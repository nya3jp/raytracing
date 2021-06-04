# Ray Tracing in One Weekend

This repository contains my Rust implementation for [_Ray Tracing in One Weekend_ book series]:

- [Ray Tracing in One Weekend]
- [Ray Tracing: The Next Week]
- [Ray Tracing: The Rest of Your Life]

[_Ray Tracing in One Weekend_ book series]: https://github.com/RayTracing/raytracing.github.io
[Ray Tracing in One Weekend]: https://raytracing.github.io/books/RayTracingInOneWeekend.html
[Ray Tracing: The Next Week]: https://raytracing.github.io/books/RayTracingTheNextWeek.html
[Ray Tracing: The Rest of Your Life]: https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html

## How to run

```
cargo build --release
./target/release/raytracing --scene=one_weekend::balls --samples=100 --width=200 --output=out.png
```

## Gallery

<p>
  <img src="/images/one_weekend_final.png" height="400">
  <img src="/images/next_week_final.png" height="400">
  <img src="/images/rest_of_your_life_final.png" height="400">
  <img src="/images/portal.png" height="400">
</p>
