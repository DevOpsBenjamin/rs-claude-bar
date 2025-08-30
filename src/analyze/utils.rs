use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, hash_map::Entry};

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
          let block = create_limit_block(*start, end, &per_hour);
          result.insert(*start, block);

          // Mark all hours in this limit window as occupied
          let mut h = *start;
          while h < end {
              occupied_hours.insert(h);
              h = h + Duration::hours(1);
          }
      }

      // 2) Create individual gap blocks for free hours
      let mut hour_keys: Vec<DateTime<Utc>> = per_hour.keys().cloned().collect();
      hour_keys.sort();

      for &hour in &hour_keys {
          if !occupied_hours.contains(&hour) {
              // This is a free hour - create a 1-hour gap block
              let gap_block = create_gap_block(&[hour], &per_hour);
              result.insert(hour, gap_block);
          }
      }

      // 3) Group consecutive gap blocks, keeping only the first of each group
      let gap_keys: Vec<DateTime<Utc>> = result.iter()
          .filter_map(|(k, v)| match v.kind {
              BlockKind::Gap => Some(*k),
              _ => None,
          })
          .collect();

      group_consecutive_gaps(&mut result, gap_keys);

      result
}

fn group_consecutive_gaps(result: &mut HashMap<DateTime<Utc>, DataBlock>, mut gap_keys: Vec<DateTime<Utc>>) {
      gap_keys.sort();

      let mut i = 0;
      while i < gap_keys.len() {
          let group_start = gap_keys[i];
          let mut group_hours = vec![group_start];

          // Find consecutive hours (max 5)
          while group_hours.len() < 5 && i + 1 < gap_keys.len() {
              let current = gap_keys[i];
              let next = gap_keys[i + 1];

              if next == current + Duration::hours(1) {
                  group_hours.push(next);
                  i += 1;
              } else {
                  break;
              }
          }

          // Remove individual gap blocks from result
          for &hour in &group_hours {
              result.remove(&hour);
          }

          // Create one consolidated gap block
          let consolidated_gap = create_gap_block(&group_hours, /* per_hour from somewhere */);
          result.insert(group_start, consolidated_gap);

          i += 1;
    }
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

fn add_per_hour_to_stats(stats: &mut DataStats, ph: &PerHourBlock) {
    stats.input_tokens += ph.input_tokens as i64;
    stats.output_tokens += ph.output_tokens as i64;
    stats.cache_creation_tokens += ph.cache_creation_tokens as i64;
    stats.cache_read_tokens += ph.cache_read_tokens as i64;
    stats.total_tokens += calculate_total_tokens(ph);
    stats.assistant_messages += ph.assistant_messages as i64;
    stats.user_messages += ph.user_messages as i64;
    stats.total_content_length += ph.total_content_length as i64;
    stats.entry_count += ph.entry_count as i64;
}

// Create a limit block from per-hour data
fn create_limit_block(start: DateTime<Utc>, end: DateTime<Utc>, per_hour: &HashMap<DateTime<Utc>, PerHourBlock>) -> DataBlock {
    let mut stats = DataStats::default();
    let mut min_timestamp = end;
    let mut max_timestamp = start;
    let mut seen = false;
    
    let mut current = start;
    while current < end {
        if let Some(ph) = per_hour.get(&current) {
            add_per_hour_to_stats(&mut stats, ph);
            if !seen {
                min_timestamp = ph.min_timestamp;
                max_timestamp = ph.max_timestamp;
                seen = true;
            } else {
                if ph.min_timestamp < min_timestamp { min_timestamp = ph.min_timestamp; }
                if ph.max_timestamp > max_timestamp { max_timestamp = ph.max_timestamp; }
            }
        }
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
fn create_gap_block(hours: &[DateTime<Utc>], per_hour: &HashMap<DateTime<Utc>, PerHourBlock>) -> DataBlock {
    let start = hours[0];
    let end = hours[hours.len() - 1] + Duration::hours(1);
    
    let mut stats = DataStats::default();
    let mut min_timestamp = start;
    let mut max_timestamp = start;
    let mut seen = false;
    
    for &hour in hours {
        if let Some(ph) = per_hour.get(&hour) {
            add_per_hour_to_stats(&mut stats, ph);
            if !seen {
                min_timestamp = ph.min_timestamp;
                max_timestamp = ph.max_timestamp;
                seen = true;
            } else {
                if ph.min_timestamp < min_timestamp { min_timestamp = ph.min_timestamp; }
                if ph.max_timestamp > max_timestamp { max_timestamp = ph.max_timestamp; }
            }
        }
    }
    
    DataBlock {
        kind: BlockKind::Gap,
        start,
        end,
        unlock_timestamp: None,
        min_timestamp,
        max_timestamp,
        stats,
    }
}
