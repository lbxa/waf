---
description: Crafting a Robust WAF Design
---

# WAF Design

## Requirements

A WAF blocks incoming traffic in one of two ways ([more details here](https://www.cloudflare.com/learning/ddos/glossary/web-application-firewall-waf/)):

* Blocklist which protects against known attacks and
* Allowlists that only admit pre-approved traffic.

A blocklist is analogous to a bouncer at the club that only lets people in if they fit the dress code whereas an allowlist is an exclusive club where only members on the invite list are allowed in.

There are benefits to either of these implementations and that’s why most WAFs come as hybrid models. Due to time constants, I will only implement a blocklist WAF for now which will scan for the following attacks: SQLi and XSS. Will be adding the allowlist to the roadmap.

## Implementation

There are three ways a WAF can be implemented

1. **Reverse Proxy**: Implement the WAF as a reverse proxy server, such as NGINX or Apache with a Rust-based module or middleware.
2. **Library**: Integrate the WAF logic directly into your web application code using a Rust library.
3. **Standalone Binary**: Develop a standalone Rust binary that acts as a WAF and sits in front of your web application server, inspecting and filtering incoming traffic.

| Integration Option  | Pros                                                                                                                                                                                                                                                                                                                                                                        | Cons                                                                                                                                                                                                                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Reverse Proxy       | <p>Works at the network level, intercepting requests before they reach the web server.</p><p></p><p>Offers a clear separation between the WAF and the web server, making it easy to switch WAF implementations or configurations.</p><p></p><p>Can be used with different web servers and technologies.</p>                                                                 | <p>Requires configuration of web server settings and potentially additional server hardware.</p><p></p><p>Can introduce a level of complexity in routing and load balancing if multiple web servers are involved.</p><p></p><p>Limited to HTTP/HTTPS traffic.</p>                                         |
| Library Integration | <p>Direct integration with the application code allows for fine-grained control over request processing.</p><p></p><p>Easier to access and modify the application's data structures and logic.</p><p></p><p>Can be used with non-HTTP protocols or non-standard web applications.</p>                                                                                       | <p>Complex to implement and maintain, as the WAF logic is tightly coupled with the application.</p><p></p><p>May require substantial code changes if the web application is already developed.</p><p></p><p>Difficult to change or switch WAF implementations without extensive code updates.</p>         |
| Standalone Binary   | <p>Provides a dedicated, isolated layer for filtering traffic before it reaches the web server.</p><p></p><p>Can be deployed without altering the existing application code or server configuration.</p><p></p><p>Suitable for protecting multiple web servers or services behind a single entry point.</p><p></p><p>Offers flexibility for scaling and load balancing.</p> | <p>Requires additional network configuration to route traffic through the standalone WAF.</p><p></p><p>May add latency to requests as they pass through the extra layer.</p><p></p><p>Requires separate deployment and maintenance from the application, potentially leading to operational overhead.</p> |

