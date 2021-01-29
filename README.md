# Silma

A Rust library for Computer Vision. Too much missing right now to be useful, WIP. Build using [`image`][1] and [`nalgebra`][2]

Meant as a way for me to learn CV things, and I will implement as I learn.

Current list of things I want to add:
- [ ] Generalize the `Filter` struct to instead operate on pointers, not matrix slices
- [ ] Functions for convolution with circular kernel, 1-D convolution
- [ ] Add the following blurring filters:
    - [X] Gaussian
    - [X] Boxed
    - [ ] Mean
    - [ ] Bilateral
- [ ] More ergonomic image handling (currently requires users to directly use [`image`][1] and [`nalgebra`][2] to allow image operations)
- [ ] Optimize filters (eg: to Gaussian stuff in 1-D once horizontal and once vertical)
- [ ] Thresholding and transforms
- [ ] Edge detection (Canny?)
- [ ] Corner detection (Harris?)
- [ ] Face detection
- [ ] Hough circle and line detection
- [ ] Image segmentation (Watershed?)
- [ ] Some ML stuff
- [ ] Performance improvements

### Goals
- Keep code very readable, with plenty comments
- Implement as I learn
- Abstract things out to simpler functions, and each feature's function/struct to be enough to explain what's going on

### Non-goals
- Super fast industry standard library (you have OpenCV and other awesome things for that)

### License

Dual licensed under MIT license and the Apache License (Version 2.0), similar to Rust's license.

[1]: [https://crates.io/crates/image]
[2]: [https://crates.io/crates/nalgebra]
