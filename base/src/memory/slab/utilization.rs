use crate::memory::buddy::order_to_size;

use super::{Object, SlabDescriptor};

pub(super) struct SlabUtilization {
    order: u8,
    num_objects: usize,
    remaining: usize,
    score: usize,
}

const MAX_ORDER: u8 = 5;

const MAX_COUNT: usize = (order_to_size(MAX_ORDER) - core::mem::size_of::<SlabDescriptor>())
    / core::mem::size_of::<Object>();

impl SlabUtilization {
    pub(super) const fn find_best(object_size: usize, padding_size: usize) -> (u8, usize, usize) {
        let mut best: Option<SlabUtilization> = None;
        let mut order = 0;
        while order < MAX_ORDER {
            let utilization = SlabUtilization::calculate(order, object_size, padding_size);

            if utilization.remaining == 0 {
                best = Some(utilization);
                break;
            }

            best = Some(match best {
                Some(best) => match utilization.score < best.score {
                    true => utilization,
                    false => best,
                },
                None => utilization,
            });

            order += 1;
        }

        let best = best.unwrap();
        (best.order, best.num_objects, best.remaining)
    }

    const fn calculate(order: u8, object_size: usize, padding_size: usize) -> Self {
        let mut remaining = order_to_size(order) - core::mem::size_of::<SlabDescriptor>();
        let mut num_objects = 0;

        while remaining >= object_size {
            num_objects += 1;
            remaining -= object_size;

            if remaining >= object_size + padding_size {
                remaining -= padding_size;
            }
        }

        let score = (remaining * MAX_COUNT) / num_objects;

        SlabUtilization {
            order,
            num_objects,
            remaining,
            score,
        }
    }
}
