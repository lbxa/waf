---
description: NGINX powers the internet
---

# NGINX

NGINX is one of the most prolific web servers used in the world. The core functionality of nginx is quite simple and focuses on network protocols whilst the main grunt work is delegated to **modules**. Anything specific that needs to be done by NGINX is most likely executed by a module one of three module types:

1. Handlers
2. Filters
3. Load balancers

Example built-in modules can be found directly in the source code of the NGINX project at [nginx.org/source](https://lxr.nginx.org/source/xref/nginx/src/http/modules/).

A typical NGINX processing cycle goes as follows:

1. The client sends an HTTP request
2. Nginx chooses the appropriate handler based on the location config
3. (if applicable) load-balancer picks a backend server
4. The handler does its thing and passes each output buffer to the first filter
5. The first filter passes the output to the second filter
6. Second to third → third to fourth → etc.
7. The final response sent to the client

NGINX modules are mostly compiled statically with the rest of the NGINX source code. However, as of NGINX 1.9.11 dynamic modules are now supported. This means newly written modules can be compiled separately to NGINX itself and be loaded into the `nginx.conf` file whenever using the `load_module` directive.&#x20;
