struct event {
  uint32_t value;
  uint8_t probability;
};

struct pattern {
  struct event* events;
  uint32_t length;
};

struct pattern transform(char* input, uint32_t subdivision);