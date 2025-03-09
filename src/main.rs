mod puzzle_1_length_limit;
mod puzzle2_grav_wave;
mod puzzle_3_pass_checker;

fn main() {
    puzzle_1_length_limit::length_limit();
    puzzle2_grav_wave::get_time_gravitational_wave_detected();
    puzzle_3_pass_checker::count_valid_passwords();
}