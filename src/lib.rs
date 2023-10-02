use eve_item_parser::{format_x, lookup_id, parse_with_id, ItemWithId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    left_items: Vec<ItemWithId>,
    right_items: Vec<ItemWithId>,
    left_missing: Vec<ItemWithId>,
    right_missing: Vec<ItemWithId>,
    left_missing_formatted: String,
    right_missing_formatted: String,
}

fn count_by_id(items: Vec<ItemWithId>) -> HashMap<u64, i64> {
    let mut totals: HashMap<u64, i64> = HashMap::new();
    for item in items.iter() {
        match totals.get(&item.type_id) {
            Some(count) => totals.insert(item.type_id, count + item.quantity),
            None => totals.insert(item.type_id, item.quantity),
        };
    }
    return totals;
}

fn sub_counts(left: HashMap<u64, i64>, right: HashMap<u64, i64>) -> HashMap<u64, i64> {
    let mut remaining = left.clone();
    for (type_id, quantity) in right.iter() {
        match remaining.get(type_id) {
            Some(count) => remaining.insert(*type_id, count - quantity),
            None => remaining.insert(*type_id, -quantity),
        };
    }
    return remaining;
}

fn diff_to_missing(diff: HashMap<u64, i64>) -> Result<Vec<ItemWithId>, String> {
    diff.iter()
        .filter(|(_, diff_quantity)| **diff_quantity > 0)
        .map(|(id, diff_quantity)| {
            let type_name = lookup_id(*id).ok_or(format!("failed to look up id {}", id))?;
            Ok(ItemWithId {
                type_id: *id,
                quantity: *diff_quantity,
                type_name,
            })
        })
        .collect()
}

pub fn diff(left_raw: &str, right_raw: &str) -> Result<DiffResult, String> {
    let left = parse_with_id(left_raw)?;
    let right = parse_with_id(right_raw)?;

    let left_totals = count_by_id(left.clone());
    let right_totals = count_by_id(right.clone());

    let left_minus_right = sub_counts(left_totals.clone(), right_totals.clone());
    let right_minus_left = sub_counts(right_totals, left_totals);

    let left_missing = diff_to_missing(right_minus_left)?;
    let right_missing = diff_to_missing(left_minus_right)?;

    Ok(DiffResult {
        left_items: left,
        right_items: right,
        left_missing: left_missing.clone(),
        right_missing: right_missing.clone(),
        left_missing_formatted: format_x(left_missing),
        right_missing_formatted: format_x(right_missing),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_simple() {
        let left_raw = "Paladin x1
Golem x2";
        let right_raw = "Golem x2
Harpy x3
";

        let mut result = diff(left_raw, right_raw).unwrap();

        result
            .left_missing
            .sort_by(|a, b| b.type_id.cmp(&a.type_id));
        result
            .right_missing
            .sort_by(|a, b| b.type_id.cmp(&a.type_id));

        assert_eq!(
            result.left_missing,
            vec![ItemWithId {
                type_name: String::from("Harpy"),
                quantity: 3,
                type_id: 11381,
            }]
        );
        assert_eq!(
            result.right_missing,
            vec![ItemWithId {
                type_name: String::from("Paladin"),
                quantity: 1,
                type_id: 28659,
            }]
        );
        println!("{}", result.left_missing_formatted);
        println!("{}", result.right_missing_formatted);
    }

    #[test]
    fn diff_complex() {
        let left_raw = "Paladin x1
Golem x2
Imperial Navy Acolyte x5
";
        let right_raw = "Golem x2
Harpy x3
Imperial Navy Acolyte x8
";

        let mut result = diff(left_raw, right_raw).unwrap();

        result
            .left_missing
            .sort_by(|a, b| b.type_id.cmp(&a.type_id));
        result
            .right_missing
            .sort_by(|a, b| b.type_id.cmp(&a.type_id));

        assert_eq!(
            result.left_missing,
            vec![
                ItemWithId {
                    type_name: String::from("Imperial Navy Acolyte"),
                    quantity: 3,
                    type_id: 31864,
                },
                ItemWithId {
                    type_name: String::from("Harpy"),
                    quantity: 3,
                    type_id: 11381,
                },
            ]
        );
        assert_eq!(
            result.right_missing,
            vec![ItemWithId {
                type_name: String::from("Paladin"),
                quantity: 1,
                type_id: 28659,
            }]
        );
        println!("{}", result.left_missing_formatted);
        println!("{}", result.right_missing_formatted);
    }
}
