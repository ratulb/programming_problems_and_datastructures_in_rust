///Gas filling and completing circular tour problem

pub fn can_complete_tour(distances: &[i32], refuels: &[i32]) -> bool {
    let stations = refuels.len();
    for i in 0..stations {
        let mut reserve = 0;
        for start_at in i..stations + i {
            let station = start_at % stations;
            if reserve + refuels[station] < distances[station] {
                break;
            } else {
                reserve += refuels[station] - distances[station];
            }
            if stations + i - start_at == 1 {
                return true;
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::can_complete_tour;
    #[test]
    fn test_can_complete_tour_1() {
        assert_eq!(
            can_complete_tour(&[5, 6, 7, 8, 5, 4], &[6, 7, 4, 10, 6, 5]),
            true
        );
    }
    #[test]
    fn test_can_complete_tour_2() {
        assert_eq!(can_complete_tour(&[3, 4, 5, 1, 2], &[1, 2, 3, 4, 5]), true);
    }
}
