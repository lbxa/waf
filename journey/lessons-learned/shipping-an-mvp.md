---
description: It's important for an MVP to be an MVP, not a v10
---

# Shipping an MVP

In my opinion, one of the hardest things dev and product teams have to decide is where to draw the line in the sand on the criteria for an MVP. Once the line gets moved even once, it's very hard to keep it from shifting further.&#x20;

I didn't want that to impact this project. I only had roughly three weeks to get something working and presentable so I didn't have enough time to implement a robust rule engine, CSRF prevention and other very important security features every WAF _should_ have.&#x20;

The bright side of not implementing the aforementioned features was that I could focus more time on perfecting the core architecture and stack. This meant Raft would only ship v1 with SQLi and XSS prevention.

I even took a further measure to leverage external libraries to handle the SQLi and XSS parsing  (check out the references section) so I could focus my time on learning Rust to build a stronger foundation for an open-source project.

{% hint style="info" %}
This decision might come back to bite in the future if support for libinjection is dropped (much like NAXSI support was on Nov 8th 2023). It's not the end of the world, since building those libraries from scratch in Rust might not take that long. It's also not ideal to freeze a stable version of the library and continue to interface with it's C implementation via Rust FFI bindings.
{% endhint %}
