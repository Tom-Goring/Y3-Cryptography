#pragma OPENCL EXTENSION cl_khr_byte_addressable_store : enable

#define ROTLEFT(a, b) ((a << b) | (a >> (32 - b)))

#define ALPHABET_SIZE 36
#define PASSWORD_SIZE 6

typedef unsigned char BYTE; // 8-bit byte
typedef unsigned int WORD;  // 32-bit word, change to "long" for 16-bit machines

typedef struct SHA1Context {
  BYTE data[64];
  WORD datalen;
  unsigned long long bitlen;
  WORD state[5];
  WORD k[4];
} SHA1Context;

void sha1_init(SHA1Context *ctx);
void sha1_update(SHA1Context *ctx, const BYTE data[], size_t len);
void sha1_final(SHA1Context *ctx, BYTE hash[]);

int indices_to_string(int *indices, char *string, int len,
                      __global char *alphabet);
bool increment_indices(char *indices, int len, int amount);

__kernel void add(__global char *output, __global char *alphabet,
                  __global BYTE *target, __global BYTE *done) {
  if (*done != 1) {
    int indices[] = {-1, -1, -1, -1, -1, -1};
    BYTE string[] = {"\0\0\0\0\0\0"};
    char digest[40];

    // increment our indices until we get to the value we want this thread to
    // test

    int len = 6;
    int amount = get_global_id(0) + 1;

    if (amount == 0) {
      return;
    }

    int carry = amount;

    for (int i = 0; i < len; i++) {
      int position = len - 1 - i;
      if (carry == 0) {
        break;
      }

      int current_value = indices[position];
      int new_value = current_value + carry;

      if (new_value >= ALPHABET_SIZE) {
        carry = new_value / ALPHABET_SIZE;
        new_value = new_value % ALPHABET_SIZE;
      } else {
        carry = 0;
      }

      indices[position] = new_value;
    }

    if (carry != 0) {
      printf("Overflow");
    }

    // finished incrementing indices

    int string_len = indices_to_string(indices, string, 6, alphabet);

    // now we want to sha1 our string
    struct SHA1Context context;
    sha1_init(&context);
    sha1_update(&context, string, string_len);
    BYTE hash[20];
    sha1_final(&context, hash);

    // check to see if hash is equivalent
    int pass = 1;
    for (int i = 0; i < 20; i++) {
      pass = pass && (hash[i] == target[i]);
    }

    if (pass == 1) {
      *done = 1;
      for (int i = 0; i < 6; i++) {
        output[i] = string[i];
      }
    }
  }
}

int indices_to_string(int *indices, char *string, int len,
                      __global char *alphabet) {
  int num_chars = 0;
  for (int i = 0; i < len; i++) {
    if (indices[i] != -1) {
      string[num_chars++] = alphabet[indices[i]];
    }
  }
  string[num_chars] = '\0';

  return num_chars;
}

void sha1_transform(SHA1Context *ctx, const BYTE data[]) {
  WORD a, b, c, d, e, i, j, t, m[80];

  for (i = 0, j = 0; i < 16; ++i, j += 4)
    m[i] = (data[j] << 24) + (data[j + 1] << 16) + (data[j + 2] << 8) +
           (data[j + 3]);
  for (; i < 80; ++i) {
    m[i] = (m[i - 3] ^ m[i - 8] ^ m[i - 14] ^ m[i - 16]);
    m[i] = (m[i] << 1) | (m[i] >> 31);
  }

  a = ctx->state[0];
  b = ctx->state[1];
  c = ctx->state[2];
  d = ctx->state[3];
  e = ctx->state[4];

  for (i = 0; i < 20; ++i) {
    t = ROTLEFT(a, 5) + ((b & c) ^ (~b & d)) + e + ctx->k[0] + m[i];
    e = d;
    d = c;
    c = ROTLEFT(b, 30);
    b = a;
    a = t;
  }
  for (; i < 40; ++i) {
    t = ROTLEFT(a, 5) + (b ^ c ^ d) + e + ctx->k[1] + m[i];
    e = d;
    d = c;
    c = ROTLEFT(b, 30);
    b = a;
    a = t;
  }
  for (; i < 60; ++i) {
    t = ROTLEFT(a, 5) + ((b & c) ^ (b & d) ^ (c & d)) + e + ctx->k[2] + m[i];
    e = d;
    d = c;
    c = ROTLEFT(b, 30);
    b = a;
    a = t;
  }
  for (; i < 80; ++i) {
    t = ROTLEFT(a, 5) + (b ^ c ^ d) + e + ctx->k[3] + m[i];
    e = d;
    d = c;
    c = ROTLEFT(b, 30);
    b = a;
    a = t;
  }

  ctx->state[0] += a;
  ctx->state[1] += b;
  ctx->state[2] += c;
  ctx->state[3] += d;
  ctx->state[4] += e;
}

void sha1_init(SHA1Context *ctx) {
  ctx->datalen = 0;
  ctx->bitlen = 0;
  ctx->state[0] = 0x67452301;
  ctx->state[1] = 0xEFCDAB89;
  ctx->state[2] = 0x98BADCFE;
  ctx->state[3] = 0x10325476;
  ctx->state[4] = 0xc3d2e1f0;
  ctx->k[0] = 0x5a827999;
  ctx->k[1] = 0x6ed9eba1;
  ctx->k[2] = 0x8f1bbcdc;
  ctx->k[3] = 0xca62c1d6;
}

void sha1_update(SHA1Context *ctx, const BYTE data[], size_t len) {
  size_t i;

  for (i = 0; i < len; ++i) {
    ctx->data[ctx->datalen] = data[i];
    ctx->datalen++;
    if (ctx->datalen == 64) {
      sha1_transform(ctx, ctx->data);
      ctx->bitlen += 512;
      ctx->datalen = 0;
    }
  }
}

void sha1_final(SHA1Context *ctx, BYTE hash[]) {
  WORD i;

  i = ctx->datalen;

  // Pad whatever data is left in the buffer.
  if (ctx->datalen < 56) {
    ctx->data[i++] = 0x80;
    while (i < 56)
      ctx->data[i++] = 0x00;
  } else {
    ctx->data[i++] = 0x80;
    while (i < 64)
      ctx->data[i++] = 0x00;
    sha1_transform(ctx, ctx->data);
    for (int i = 0; i < 56; i++) {
      ctx->data[i] = 0;
    }
    // memset(ctx->data, 0, 56);
  }

  // Append to the padding the total message's length in bits and transform.
  ctx->bitlen += ctx->datalen * 8;
  ctx->data[63] = ctx->bitlen;
  ctx->data[62] = ctx->bitlen >> 8;
  ctx->data[61] = ctx->bitlen >> 16;
  ctx->data[60] = ctx->bitlen >> 24;
  ctx->data[59] = ctx->bitlen >> 32;
  ctx->data[58] = ctx->bitlen >> 40;
  ctx->data[57] = ctx->bitlen >> 48;
  ctx->data[56] = ctx->bitlen >> 56;
  sha1_transform(ctx, ctx->data);

  // Since this implementation uses little endian byte ordering and MD uses big
  // endian, reverse all the bytes when copying the final state to the output
  // hash.
  for (i = 0; i < 4; ++i) {
    hash[i] = (ctx->state[0] >> (24 - i * 8)) & 0x000000ff;
    hash[i + 4] = (ctx->state[1] >> (24 - i * 8)) & 0x000000ff;
    hash[i + 8] = (ctx->state[2] >> (24 - i * 8)) & 0x000000ff;
    hash[i + 12] = (ctx->state[3] >> (24 - i * 8)) & 0x000000ff;
    hash[i + 16] = (ctx->state[4] >> (24 - i * 8)) & 0x000000ff;
  }
}