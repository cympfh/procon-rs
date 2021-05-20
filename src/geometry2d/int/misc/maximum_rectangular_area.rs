/// Maximum Rectangular Area in a Histogram, 極大長方形
pub fn maximum_rect_area(xs: &[u64]) -> u64 {
    let mut maxsize = 0;
    let mut stack = vec![];
    for (i, x) in xs.iter().chain(&[0]).enumerate() {
        if stack.is_empty() {
            stack.push((i, x));
            continue;
        }
        match stack.last().unwrap().1 {
            lastheight if lastheight < x => stack.push((i, x)),
            lastheight if lastheight > x => {
                let mut target = i;
                while let Some((pos, height)) = stack.pop() {
                    if height < x {
                        stack.push((pos, height));
                        break;
                    }
                    let area = height * (i - pos) as u64;
                    maxsize = maxsize.max(area);
                    target = pos;
                }
                stack.push((target, x));
            }
            _ => {}
        }
    }
    maxsize
}

#[cfg(test)]
mod test_maximum_rect {
    use crate::geometry2d::int::misc::maximum_rectangular_area::*;
    #[test]
    fn it_works() {
        assert_eq!(maximum_rect_area(&vec![2, 1, 3, 5, 3, 4, 2, 1]), 12);
        assert_eq!(maximum_rect_area(&vec![200, 1, 3, 5, 3, 4, 2, 1]), 200);
        assert_eq!(maximum_rect_area(&vec![2, 0, 1]), 2);
        assert_eq!(maximum_rect_area(&vec![2, 4, 4, 9, 4, 9]), 20);
        assert_eq!(maximum_rect_area(&vec![200, 4, 4, 9, 4, 9]), 200);
    }
}
