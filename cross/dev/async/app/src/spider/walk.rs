use embassy_time::Delay;
use animate::{animate, Move};
use s90::Servo;
use s90::F64Ext;

pub async fn walk<S: Servo>(
    knees1: (&mut S, &mut S),
    knees2: (&mut S, &mut S),
    hips1:  (&mut S, &mut S),
    hips2:  (&mut S, &mut S),
    sleep: &mut Delay,
    steps: u32,
    speed: u32,
) {

    for _ in 0..steps {
        animate(
            &mut [
                Move::new(knees1.0, 100.0_f64.degrees()),
                Move::new(knees1.1, 100.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(hips1.0, 20.0_f64.degrees()),
                Move::new(hips1.1, 70.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(knees1.0, 180.0_f64.degrees()),
                Move::new(knees1.1, 180.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(hips1.0, 45.0_f64.degrees()),
                Move::new(hips1.1, 45.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(knees1.0, 165.0_f64.degrees()),
                Move::new(knees1.1, 165.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        //**********
        animate(
            &mut [
                Move::new(knees2.0, 100.0_f64.degrees()),
                Move::new(knees2.1, 100.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(hips2.0, 70.0_f64.degrees()),
                Move::new(hips2.1, 20.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;

        animate(
            &mut [
                Move::new(knees2.0, 180.0_f64.degrees()),
                Move::new(knees2.1, 180.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(hips2.0, 45.0_f64.degrees()),
                Move::new(hips2.1, 45.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
        animate(
            &mut [
                Move::new(knees2.0, 165.0_f64.degrees()),
                Move::new(knees2.1, 165.0_f64.degrees()),
            ],
            speed,
            sleep,
        ).await;
    }
}