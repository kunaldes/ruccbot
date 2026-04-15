use crate::{Context, Error};

use lazy_static::lazy_static;
use rand::{
    RngExt,
    seq::{IteratorRandom, SliceRandom},
};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\s*(\d+)\s*-\s*(\d+)\s*$").unwrap();
}

fn do_choose(items: &str) -> Option<String> {
    if let Some(captures) = RE.captures(items) {
        let (_, [i1, i2]) = captures.extract();
        if let (Ok(num1), Ok(num2)) = (i1.parse(), i2.parse()) {
            let min: i32 = std::cmp::min(num1, num2);
            let max: i32 = std::cmp::max(num1, num2);
            return Some(rand::rng().random_range(min..=max).to_string());
        }
    }
    if items.contains(',') {
        items
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
            .choose(&mut rand::rng())
            .map(|x| x.to_string())
    } else {
        items
            .split_whitespace()
            .choose(&mut rand::rng())
            .map(|x| x.to_string())
    }
}

fn do_order(items: &str) -> Option<String> {
    let mut foo: Vec<&str>;
    if items.contains(',') {
        foo = items
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim())
            .collect();
    } else {
        foo = items.split_whitespace().collect();
    }

    if foo.is_empty() {
        return None;
    }
    foo.shuffle(&mut rand::rng());
    Some(foo.join(", "))
}

/// Choose a single entry from a list.
///
/// The list can be either a range of positive integers, a comma
/// separated list or a space separated list.
#[poise::command(prefix_command, aliases("c"))]
pub async fn choose(
    ctx: Context<'_>,
    #[description = "The list to choose from."]
    #[rest]
    items: Option<String>,
) -> Result<(), Error> {
    if let Some(choice) = do_choose(&items.unwrap_or_default()) {
        ctx.say(choice).await?;
    }
    Ok(())
}

/// Shuffles the input list.
///
/// The list must be either comma separated or space separated.
/// This command does not (and will never) support ordering a numeric range.
#[poise::command(prefix_command, aliases("o"))]
pub async fn order(
    ctx: Context<'_>,
    #[description = "The list to reorder."]
    #[rest]
    items: Option<String>,
) -> Result<(), Error> {
    if let Some(ordering) = do_order(&items.unwrap_or_default()) {
        ctx.say(ordering).await?;
    }
    Ok(())
}
