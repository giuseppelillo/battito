struct event {
  uint32_t value;
  uint8_t probability;
};

struct event* transform(char* input, uint32_t subdivision);