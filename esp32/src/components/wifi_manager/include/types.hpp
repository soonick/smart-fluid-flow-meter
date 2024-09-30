#pragma once

struct dns_server_handle {
  TaskHandle_t task;
};

struct dns_answer {
  uint16_t ptr_offset;
  uint16_t type;
  uint16_t klass;
  uint32_t ttl;
  uint16_t addr_len;
  uint32_t ip_addr;
} __attribute__((__packed__));
