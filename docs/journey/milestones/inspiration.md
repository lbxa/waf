---
description: Choosing the right project
---

# Inspiration

I began my research by exploring general firewall software before delving into [Web Application Firewalls](https://www.cloudflare.com/learning/ddos/glossary/web-application-firewall-waf/) (WAFs). One prominent discovery was an open-source WAF called [Naxsi](https://github.com/nbs-system/naxsi), based on NGINX, which piqued my interest.

To understand more about the inner workings of firewalls, I delved into the [source code of UFW](https://code.launchpad.net/ufw), the Uncomplicated Firewall used in Debian Linux machines. To my surprise, it was predominantly written in Python, a choice that intrigued me given Python's latency issues.

In my quest for a stack that combines type-safety with NGINX, I found Naxsi, which served as a valuable example. It was built as an NGINX module, aligning with my vision. However, I recognized that I had [much to learn](https://www.cloudflare.com/learning/ddos/glossary/web-application-firewall-waf/) about the principles of WAFs in this exciting journey.
