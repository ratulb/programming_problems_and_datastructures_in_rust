/***
There are n oranges in the kitchen and you decided to eat some of these oranges
every day as follows:

Eat one orange.
If the number of remaining oranges n is divisible by 2 then you can eat n / 2 oranges.
If the number of remaining oranges n is divisible by 3 then you can eat 2 * (n / 3)
oranges.
You can only choose one of the actions per day.
***/

use std::cmp;
use std::collections::HashMap;
pub fn min_days(n: i32) -> i32 {
    let mut dp = HashMap::<i32, i32>::new();
    dp.insert(0, 0);
    dp.insert(1, 1);
    minimum_days(n, &mut dp);
    *dp.get(&n).unwrap()
}

fn minimum_days(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
    if dp.get(&n).is_some() {
        return *dp.get(&n).unwrap();
    }
    let one_day = 1 + (n % 2) + minimum_days(n / 2, dp);
    let one_or_tow_days = 1 + (n % 3) + minimum_days(n / 3, dp);
    let min_days = cmp::min(one_day, one_or_tow_days);
    dp.insert(n, min_days);
    min_days
}

#[cfg(test)]
mod tests {
    use super::min_days;
    #[test]
    fn test_min_days() {
        let result = min_days(10);
        assert_eq!(result, 4);

        let result = min_days(6);
        assert_eq!(result, 3);
    }
}
