use crate::{Context, Error};

use lazy_static::lazy_static;
use rand::{seq::SliceRandom, Rng};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^\s*(\d+)\s*-\s*(\d+)\s*$").unwrap();
}

fn split_input(input: String) -> Vec<String> {
    if input.contains(',') {
        input
            .split(',')
            .skip_while(|x| x.is_empty())
            .map(|x| x.trim().into())
            .collect()
    } else {
        input.split_whitespace().map(|x| x.into()).collect()
    }
}

fn do_choose(items: String) -> Option<String> {
    if let Some(captures) = RE.captures(items.as_str()) {
        let (_, [i1, i2]) = captures.extract();
        if let (Ok(num1), Ok(num2)) = (i1.parse(), i2.parse()) {
            let [min, max]: [i32; 2] = std::cmp::minmax(num1, num2);
            return Some(rand::thread_rng().gen_range(min..=max).to_string());
        }
    }
    let foo: Vec<String> = split_input(items);
    foo.choose(&mut rand::thread_rng()).cloned()
}

fn do_order(items: String) -> Option<String> {
    let mut foo = split_input(items);
    if foo.is_empty() {
        return None;
    }
    foo.shuffle(&mut rand::thread_rng());
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
    if let Some(choice) = do_choose(items.unwrap_or_default()) {
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
    #[description = "The list to choose from."]
    #[rest]
    items: Option<String>,
) -> Result<(), Error> {
    if let Some(ordering) = do_order(items.unwrap_or_default()) {
        ctx.say(ordering).await?;
    }
    Ok(())
}
