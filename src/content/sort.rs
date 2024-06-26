use std::cmp::Ordering;

use super::title::WithTitle;

pub(crate) fn by_title<T: WithTitle>(a: &&T, b: &&T) -> Ordering {
    a.title().cmp(&b.title()).reverse()
}
