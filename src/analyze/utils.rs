use chrono::{DateTime, Utc, Duration};
use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{
    analyze::{DataBlock, DataStats, BlockKind, LimitBlock},
    cache::{CacheInfo, PerHourBlock}
};

// STEP 1: Find FIXED 5-hour windows from limit messages
/// Flatten all per-hour usage across all files into a single map keyed by hour start.
fn build_per_hour_agg(cache: &CacheInfo) -> (HashMap<DateTime<Utc>, LimitBlock>, HashMap<DateTime<Utc>, PerHourBlock>) {
    let mut limit_blocks: HashMap<DateTime<Utc>, LimitBlock> = HashMap::new();
    let mut per_hour_block: HashMap<DateTime<Utc>, PerHourBlock> = HashMap::new();

    for (_folder, folder) in &cache.folders {
        for (_file, file) in &folder.files {
            // Aggregate per-hour blocks
            for (hour_start, ph) in &file.per_hour {
                match per_hour_block.entry(*hour_start) {
                    Entry::Vacant(entry) => {
                        entry.insert(ph.clone());
                    },
                    Entry::Occupied(mut entry) => {
                        let block = entry.get_mut();
                        if ph.min_timestamp < block.min_timestamp {
                            block.min_timestamp = ph.min_timestamp;
                        }
                        if ph.max_timestamp > block.max_timestamp {
                            block.max_timestamp = ph.max_timestamp;
                        }
                        block.input_tokens += ph.input_tokens;
                        block.output_tokens += ph.output_tokens;
                        block.cache_creation_tokens += ph.cache_creation_tokens;
                        block.cache_read_tokens += ph.cache_read_tokens;
                        block.assistant_messages += ph.assistant_messages;
                        block.user_messages += ph.user_messages;
                        block.total_content_length += ph.total_content_length;
                        block.entry_count += ph.entry_count;
                    }
                }
            }
            
            // Collect limit blocks
            for (_ts, block) in &file.blocks {
                if let Some(unlock) = block.unlock_timestamp {
                    let start = unlock - Duration::hours(5);
                    limit_blocks.entry(start).or_insert(LimitBlock {
                        unlock_timestamp: unlock,
                    });
                }
            }
        }
    }

    (limit_blocks, per_hour_block)
}

/// Build limit windows from cache limits and populate aggregates from pre-aggregated per-hour data.
pub fn analyze_blocks(cache: &CacheInfo) -> HashMap<DateTime<Utc>, DataBlock> {
      let (limit_blocks, per_hour) = build_per_hour_agg(cache);
      let mut result: HashMap<DateTime<Utc>, DataBlock> = HashMap::new();
      
      // 1) Create limit blocks and mark their hours as "occupied"
      let mut occupied_hours = HashSet::new();
      
      for (start, lb) in &limit_blocks {
          let end = lb.unlock_timestamp;
          let block = create_limit_block(*start, end, &per_hour, &mut occupied_hours);
          result.insert(*start, block);
      }

      // 2) Create individual gap blocks for free hours
      let hour_keys: Vec<DateTime<Utc>> = per_hour.keys().cloned().collect();

      for &hour in &hour_keys {
          if !occupied_hours.contains(&hour) {
              // This is a free hour - create a 1-hour gap block
              let gap_block = create_gap_block(&hour, &per_hour);
              result.insert(hour, gap_block);
          }
      }

      group_consecutive_gaps(&mut result);

      result
}

fn group_consecutive_gaps(result: &mut HashMap<DateTime<Utc>, DataBlock>) {
    let mut keys: Vec<DateTime<Utc>> = result.iter()
        .filter_map(|(k, v)| match v.kind {
            BlockKind::Gap => Some(*k),
            _ => None,
        })
        .collect();

   keys.sort();
   let grouped = group_consecutive_datetimes(keys);

   for group in grouped {
       if group.len() <= 1 {
           continue;
       }

       let first_key = group[0];

        let mut merged_block = result.get(&first_key).unwrap().clone(); // Clone au lieu de get_mut

        for &key in &group[1..] {
            let block_to_merge = result.remove(&key).unwrap();
            merge_data_blocks(&mut merged_block, &block_to_merge);
        }
        
        // Réinsérer le bloc fusionné
        result.insert(first_key, merged_block);
   }
}

fn merge_data_blocks(merged_block: &mut DataBlock, block_to_merge: &DataBlock) {
    merge_stats(&mut merged_block.stats, &block_to_merge.stats);
    merged_block.end = block_to_merge.end;
    merged_block.max_timestamp = block_to_merge.max_timestamp;
}

fn group_consecutive_datetimes(datetimes: Vec<DateTime<Utc>>) -> Vec<Vec<DateTime<Utc>>> {
    if datetimes.is_empty() {
        return Vec::new();
    }
    
    let mut groups = Vec::new();
    let mut current_group = vec![datetimes[0]];
    
    for i in 1..datetimes.len() {
        let current = datetimes[i];
        let previous = current_group.last().unwrap();
        
        // Vérifier si l'écart est <= 1 heure et si le groupe n'est pas plein
        let time_diff = current.signed_duration_since(*previous);
        let is_consecutive = time_diff <= Duration::hours(1) && time_diff >= Duration::zero();
        let group_not_full = current_group.len() < 5;
        
        if is_consecutive && group_not_full {
            // Ajouter au groupe actuel
            current_group.push(current);
        } else {
            // Finaliser le groupe actuel et en commencer un nouveau
            groups.push(current_group);
            current_group = vec![current];
        }
    }
    
    // Ajouter le dernier groupe
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    
    groups
}

//HELPERS
fn calculate_total_tokens(ph: &PerHourBlock) -> i64 {
    ph.input_tokens as i64 
    + ph.output_tokens as i64 
    + ph.cache_creation_tokens as i64 
    + ph.cache_read_tokens as i64
}

fn create_stats_from_per_hour(ph: &PerHourBlock) -> DataStats {
    DataStats {
        input_tokens: ph.input_tokens as i64,
        output_tokens: ph.output_tokens as i64,
        cache_creation_tokens: ph.cache_creation_tokens as i64,
        cache_read_tokens: ph.cache_read_tokens as i64,
        total_tokens: calculate_total_tokens(ph),
        assistant_messages: ph.assistant_messages as i64,
        user_messages: ph.user_messages as i64,
        total_content_length: ph.total_content_length as i64,
        entry_count: ph.entry_count as i64,
    }
}

fn merge_stats(stats: &mut DataStats, to_add: &DataStats) {
    stats.input_tokens += to_add.input_tokens as i64;
    stats.output_tokens += to_add.output_tokens as i64;
    stats.cache_creation_tokens += to_add.cache_creation_tokens as i64;
    stats.cache_read_tokens += to_add.cache_read_tokens as i64;
    stats.total_tokens += to_add.total_tokens;
    stats.assistant_messages += to_add.assistant_messages as i64;
    stats.user_messages += to_add.user_messages as i64;
    stats.total_content_length += to_add.total_content_length as i64;
    stats.entry_count += to_add.entry_count as i64;
}

// Create a limit block from per-hour data
fn create_limit_block(
    start: DateTime<Utc>, 
    end: DateTime<Utc>, 
    per_hour: &HashMap<DateTime<Utc>, PerHourBlock>,
    occupied_hours: &mut HashSet<DateTime<Utc>>,
) -> DataBlock {
    let mut stats = DataStats::default();
    let mut min_timestamp = end;
    let mut max_timestamp = start;
    
    let mut current = start;
    while current < end {
        if let Some(ph) = per_hour.get(&current) {
            let current_stats = create_stats_from_per_hour(ph);
            merge_stats(&mut stats, &current_stats);
            // Always true on first
            if ph.min_timestamp < min_timestamp { min_timestamp = ph.min_timestamp; }
            // Always true on first
            if ph.max_timestamp > max_timestamp { max_timestamp = ph.max_timestamp; }
        }
        occupied_hours.insert(current);
        current = current + Duration::hours(1);
    }
    
    DataBlock {
        kind: BlockKind::Limit,
        start,
        end,
        unlock_timestamp: Some(end),
        min_timestamp,
        max_timestamp,
        stats,
    }
}

// Create a gap block from consecutive hours
fn create_gap_block(
    hour: &DateTime<Utc>, 
    per_hour: &HashMap<DateTime<Utc>, PerHourBlock>
) -> DataBlock {
    let start = *hour;
    let end = start + Duration::hours(1);

    let ph = per_hour.get(hour).unwrap();

    DataBlock {
        kind: BlockKind::Gap,
        start,
        end,
        unlock_timestamp: None,
        min_timestamp: ph.min_timestamp,
        max_timestamp: ph.max_timestamp,
        stats: create_stats_from_per_hour(ph),
    }
}
