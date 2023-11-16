---
description: The success of any project greatly depends on its longevity
---

# Scale means DX

I wanted to build an open-source WAF because there's no better way to improve security vulnerabilities than making the source code public (albeit this has its own negative implications).

The success of any open-source project is dictated by my active community contribution. So the first question I asked myself when building this was

> If I'm a developer, why would I want to contribute to this project?

I was baffled by this for a while, but eventually, it became clear. I just had to think of a counterexample first. NAXSI is a popular WAF because it has very low inertia in terms of setup and maintenance. The problem was the code was ugly and written in C, making it hard to contribute to and also susceptible to many forms of binary exploits.&#x20;

There is a lot of accidental complexity (read [No Silver Bullet](https://en.wikipedia.org/wiki/No\_Silver\_Bullet)) in the NAXSI code base that would be avoided with a simpler design. Additionally if were written in a more modern and type-safe language, this could incentivise great developers to both adopt Raft and contribute to it. Overall the Developer Experience got better so the scale of the project can proportionally increase. This is why DX is the of this page.
