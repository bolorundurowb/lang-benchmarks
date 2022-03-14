# web-benchmarks
A series of tests of various web frameworks to see how they perform. I know my methodoloy is not the most scientific, but this projects serves an opportunity to learn and show minimal APIs can be setup for various languages and frameworks.

## Contributing

Feel free to send in a pull request or open a new issue with suggestions on what I can do better.

## Tools
I used [autocannon (v7.7.0)](https://www.npmjs.com/package/autocannon) for the tests. Running the following command:

```bash
autocannon -c 256 -d 60 -p 10 http://localhost:7654 
```

## Machine Specs

- OS: Windows 11 Pro (Build 22000.556)
- CPU: Ryzen 7 5700G
- RAM: 64GB (3200MHz CL22)

<br />

# Results

## Javascript

### Express
_v4.17.3_

| Stat    | 2.5%   | 50%    | 97.5%  | 99%    | Avg       | Stdev    | Max    |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Latency | 191 ms | 196 ms | 215 ms | 279 ms | 201.05 ms | 19.16 ms | 587 ms |


| Stat      | 1%      | 2.5%    | 50%     | 97.5%  | Avg     | Stdev  | Min     |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Req/Sec   | 8831    | 12279   | 12751   | 12959  | 12685.2 | 517    | 8831    |
| Bytes/Sec | 2.11 MB | 2.93 MB | 3.05 MB | 3.1 MB | 3.03 MB | 124 kB | 2.11 MB |


Req/Bytes counts sampled once per second.
No of samples: 60

764k requests in 60.06s, 182 MB read


### Fastify
_v3.27.4_


| Stat    | 2.5%  | 50%   | 97.5% | 99%   | Avg      | Stdev   | Max    |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Latency | 20 ms | 32 ms | 56 ms | 59 ms | 33.07 ms | 9.72 ms | 222 ms |

| Stat      | 1%      | 2.5%    | 50%     | 97.5%   | Avg     | Stdev   | Min     |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Req/Sec   | 57247   | 72511   | 76607   | 78399   | 76279.2 | 2763.62 | 57244   |
| Bytes/Sec | 10.1 MB | 12.8 MB | 13.5 MB | 13.8 MB | 13.4 MB | 487 kB  | 10.1 MB |


Req/Bytes counts sampled once per second.
No of samples: 60

4579k requests in 60.07s, 806 MB read

<br />

## dot NET

### C#/ASP.NET Core
_v6.0.200_


| Stat    | 2.5%  | 50%   | 97.5% | 99%   | Avg      | Stdev   | Max    |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Latency | 35 ms | 36 ms | 38 ms | 39 ms | 36.15 ms | 3.09 ms | 218 ms |


| Stat      | 1%      | 2.5%    | 50%     | 97.5%   | Avg      | Stdev   | Min     |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Req/Sec   | 57663   | 69183   | 71743   | 71743   | 71430.24 | 1839.61 | 57660   |
| Bytes/Sec | 9.46 MB | 11.3 MB | 11.8 MB | 11.8 MB | 11.7 MB  | 301 kB  | 9.46 MB |


Req/Bytes counts sampled once per second.
No of samples: 59

4215k requests in 60.4s, 691 MB read

<br/ >

## Rust

### Rocket
_v0.5.0-rc.1_


| Stat    | 2.5%  | 50%   | 97.5% | 99%   | Avg      | Stdev   | Max    |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Latency | 23 ms | 31 ms | 57 ms | 60 ms | 34.57 ms | 9.98 ms | 169 ms |


| Stat      | 1%      | 2.5%    | 50%     | 97.5% | Avg     | Stdev   | Min     |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Req/Sec   | 60415   | 66559   | 74175   | 76799 | 74220   | 3015.16 | 60410   |
| Bytes/Sec | 14.9 MB | 16.4 MB | 18.3 MB | 19 MB | 18.3 MB | 745 kB  | 14.9 MB |


Req/Bytes counts sampled once per second.
No of samples: 60

4456k requests in 61.07s, 1.1 GB read

### Actix
_v4.0.1_

_Currently have the build failing_