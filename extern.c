#include "/home/kurt/RustroverProjects/garnshared/target/debug/build/garnshared-ac4796c4e993ea7c/out/ck-install/include/ck_ring.h"

// Static wrappers

int ck_cc_ffs__extern(unsigned int x) { return ck_cc_ffs(x); }
int ck_cc_ffsl__extern(unsigned long x) { return ck_cc_ffsl(x); }
int ck_cc_ctz__extern(unsigned int x) { return ck_cc_ctz(x); }
int ck_cc_popcount__extern(unsigned int x) { return ck_cc_popcount(x); }
int ck_cc_ffsll__extern(unsigned long long v) { return ck_cc_ffsll(v); }
void ck_pr_stall__extern(void) { ck_pr_stall(); }
void ck_pr_fence_strict_atomic__extern(void) { ck_pr_fence_strict_atomic(); }
void ck_pr_fence_strict_atomic_store__extern(void) { ck_pr_fence_strict_atomic_store(); }
void ck_pr_fence_strict_atomic_load__extern(void) { ck_pr_fence_strict_atomic_load(); }
void ck_pr_fence_strict_store_atomic__extern(void) { ck_pr_fence_strict_store_atomic(); }
void ck_pr_fence_strict_load_atomic__extern(void) { ck_pr_fence_strict_load_atomic(); }
void ck_pr_fence_strict_load__extern(void) { ck_pr_fence_strict_load(); }
void ck_pr_fence_strict_load_store__extern(void) { ck_pr_fence_strict_load_store(); }
void ck_pr_fence_strict_store__extern(void) { ck_pr_fence_strict_store(); }
void ck_pr_fence_strict_store_load__extern(void) { ck_pr_fence_strict_store_load(); }
void ck_pr_fence_strict_memory__extern(void) { ck_pr_fence_strict_memory(); }
void ck_pr_fence_strict_release__extern(void) { ck_pr_fence_strict_release(); }
void ck_pr_fence_strict_acquire__extern(void) { ck_pr_fence_strict_acquire(); }
void ck_pr_fence_strict_acqrel__extern(void) { ck_pr_fence_strict_acqrel(); }
void ck_pr_fence_strict_lock__extern(void) { ck_pr_fence_strict_lock(); }
void ck_pr_fence_strict_unlock__extern(void) { ck_pr_fence_strict_unlock(); }
void ck_pr_rfo__extern(const void *m) { ck_pr_rfo(m); }
void * ck_pr_fas_ptr__extern(void *target, void *v) { return ck_pr_fas_ptr(target, v); }
double ck_pr_fas_double__extern(double *target, double v) { return ck_pr_fas_double(target, v); }
char ck_pr_fas_char__extern(char *target, char v) { return ck_pr_fas_char(target, v); }
unsigned int ck_pr_fas_uint__extern(unsigned int *target, unsigned int v) { return ck_pr_fas_uint(target, v); }
int ck_pr_fas_int__extern(int *target, int v) { return ck_pr_fas_int(target, v); }
uint64_t ck_pr_fas_64__extern(uint64_t *target, uint64_t v) { return ck_pr_fas_64(target, v); }
uint32_t ck_pr_fas_32__extern(uint32_t *target, uint32_t v) { return ck_pr_fas_32(target, v); }
uint16_t ck_pr_fas_16__extern(uint16_t *target, uint16_t v) { return ck_pr_fas_16(target, v); }
uint8_t ck_pr_fas_8__extern(uint8_t *target, uint8_t v) { return ck_pr_fas_8(target, v); }
void * ck_pr_md_load_ptr__extern(const void *target) { return ck_pr_md_load_ptr(target); }
char ck_pr_md_load_char__extern(const char *target) { return ck_pr_md_load_char(target); }
unsigned int ck_pr_md_load_uint__extern(const unsigned int *target) { return ck_pr_md_load_uint(target); }
int ck_pr_md_load_int__extern(const int *target) { return ck_pr_md_load_int(target); }
double ck_pr_md_load_double__extern(const double *target) { return ck_pr_md_load_double(target); }
uint64_t ck_pr_md_load_64__extern(const uint64_t *target) { return ck_pr_md_load_64(target); }
uint32_t ck_pr_md_load_32__extern(const uint32_t *target) { return ck_pr_md_load_32(target); }
uint16_t ck_pr_md_load_16__extern(const uint16_t *target) { return ck_pr_md_load_16(target); }
uint8_t ck_pr_md_load_8__extern(const uint8_t *target) { return ck_pr_md_load_8(target); }
void ck_pr_load_64_2__extern(const uint64_t target [2], uint64_t v [2]) { ck_pr_load_64_2(target, v); }
void ck_pr_load_ptr_2__extern(const void *t, void *v) { ck_pr_load_ptr_2(t, v); }
void ck_pr_md_load_char_16__extern(const char t [16], char v [16]) { ck_pr_md_load_char_16(t, v); }
void ck_pr_md_load_int_4__extern(const int t [4], int v [4]) { ck_pr_md_load_int_4(t, v); }
void ck_pr_md_load_uint_4__extern(const unsigned int t [4], unsigned int v [4]) { ck_pr_md_load_uint_4(t, v); }
void ck_pr_md_load_32_4__extern(const uint32_t t [4], uint32_t v [4]) { ck_pr_md_load_32_4(t, v); }
void ck_pr_md_load_16_8__extern(const uint16_t t [8], uint16_t v [8]) { ck_pr_md_load_16_8(t, v); }
void ck_pr_md_load_8_16__extern(const uint8_t t [16], uint8_t v [16]) { ck_pr_md_load_8_16(t, v); }
void ck_pr_md_store_ptr__extern(void *target, const void *v) { ck_pr_md_store_ptr(target, v); }
void ck_pr_md_store_double__extern(double *target, double v) { ck_pr_md_store_double(target, v); }
void ck_pr_md_store_char__extern(char *target, char v) { ck_pr_md_store_char(target, v); }
void ck_pr_md_store_int__extern(int *target, int v) { ck_pr_md_store_int(target, v); }
void ck_pr_md_store_uint__extern(unsigned int *target, unsigned int v) { ck_pr_md_store_uint(target, v); }
void ck_pr_md_store_64__extern(uint64_t *target, uint64_t v) { ck_pr_md_store_64(target, v); }
void ck_pr_md_store_32__extern(uint32_t *target, uint32_t v) { ck_pr_md_store_32(target, v); }
void ck_pr_md_store_16__extern(uint16_t *target, uint16_t v) { ck_pr_md_store_16(target, v); }
void ck_pr_md_store_8__extern(uint8_t *target, uint8_t v) { ck_pr_md_store_8(target, v); }
uintptr_t ck_pr_faa_ptr__extern(void *target, uintptr_t d) { return ck_pr_faa_ptr(target, d); }
char ck_pr_faa_char__extern(char *target, char d) { return ck_pr_faa_char(target, d); }
unsigned int ck_pr_faa_uint__extern(unsigned int *target, unsigned int d) { return ck_pr_faa_uint(target, d); }
int ck_pr_faa_int__extern(int *target, int d) { return ck_pr_faa_int(target, d); }
uint64_t ck_pr_faa_64__extern(uint64_t *target, uint64_t d) { return ck_pr_faa_64(target, d); }
uint32_t ck_pr_faa_32__extern(uint32_t *target, uint32_t d) { return ck_pr_faa_32(target, d); }
uint16_t ck_pr_faa_16__extern(uint16_t *target, uint16_t d) { return ck_pr_faa_16(target, d); }
uint8_t ck_pr_faa_8__extern(uint8_t *target, uint8_t d) { return ck_pr_faa_8(target, d); }
void ck_pr_inc_ptr__extern(void *target) { ck_pr_inc_ptr(target); }
bool ck_pr_inc_ptr_is_zero__extern(void *target) { return ck_pr_inc_ptr_is_zero(target); }
void ck_pr_inc_char__extern(char *target) { ck_pr_inc_char(target); }
bool ck_pr_inc_char_is_zero__extern(char *target) { return ck_pr_inc_char_is_zero(target); }
void ck_pr_inc_int__extern(int *target) { ck_pr_inc_int(target); }
bool ck_pr_inc_int_is_zero__extern(int *target) { return ck_pr_inc_int_is_zero(target); }
void ck_pr_inc_uint__extern(unsigned int *target) { ck_pr_inc_uint(target); }
bool ck_pr_inc_uint_is_zero__extern(unsigned int *target) { return ck_pr_inc_uint_is_zero(target); }
void ck_pr_inc_64__extern(uint64_t *target) { ck_pr_inc_64(target); }
bool ck_pr_inc_64_is_zero__extern(uint64_t *target) { return ck_pr_inc_64_is_zero(target); }
void ck_pr_inc_32__extern(uint32_t *target) { ck_pr_inc_32(target); }
bool ck_pr_inc_32_is_zero__extern(uint32_t *target) { return ck_pr_inc_32_is_zero(target); }
void ck_pr_inc_16__extern(uint16_t *target) { ck_pr_inc_16(target); }
bool ck_pr_inc_16_is_zero__extern(uint16_t *target) { return ck_pr_inc_16_is_zero(target); }
void ck_pr_inc_8__extern(uint8_t *target) { ck_pr_inc_8(target); }
bool ck_pr_inc_8_is_zero__extern(uint8_t *target) { return ck_pr_inc_8_is_zero(target); }
void ck_pr_dec_ptr__extern(void *target) { ck_pr_dec_ptr(target); }
bool ck_pr_dec_ptr_is_zero__extern(void *target) { return ck_pr_dec_ptr_is_zero(target); }
void ck_pr_dec_char__extern(char *target) { ck_pr_dec_char(target); }
bool ck_pr_dec_char_is_zero__extern(char *target) { return ck_pr_dec_char_is_zero(target); }
void ck_pr_dec_int__extern(int *target) { ck_pr_dec_int(target); }
bool ck_pr_dec_int_is_zero__extern(int *target) { return ck_pr_dec_int_is_zero(target); }
void ck_pr_dec_uint__extern(unsigned int *target) { ck_pr_dec_uint(target); }
bool ck_pr_dec_uint_is_zero__extern(unsigned int *target) { return ck_pr_dec_uint_is_zero(target); }
void ck_pr_dec_64__extern(uint64_t *target) { ck_pr_dec_64(target); }
bool ck_pr_dec_64_is_zero__extern(uint64_t *target) { return ck_pr_dec_64_is_zero(target); }
void ck_pr_dec_32__extern(uint32_t *target) { ck_pr_dec_32(target); }
bool ck_pr_dec_32_is_zero__extern(uint32_t *target) { return ck_pr_dec_32_is_zero(target); }
void ck_pr_dec_16__extern(uint16_t *target) { ck_pr_dec_16(target); }
bool ck_pr_dec_16_is_zero__extern(uint16_t *target) { return ck_pr_dec_16_is_zero(target); }
void ck_pr_dec_8__extern(uint8_t *target) { ck_pr_dec_8(target); }
bool ck_pr_dec_8_is_zero__extern(uint8_t *target) { return ck_pr_dec_8_is_zero(target); }
void ck_pr_neg_ptr__extern(void *target) { ck_pr_neg_ptr(target); }
bool ck_pr_neg_ptr_is_zero__extern(void *target) { return ck_pr_neg_ptr_is_zero(target); }
void ck_pr_neg_char__extern(char *target) { ck_pr_neg_char(target); }
bool ck_pr_neg_char_is_zero__extern(char *target) { return ck_pr_neg_char_is_zero(target); }
void ck_pr_neg_int__extern(int *target) { ck_pr_neg_int(target); }
bool ck_pr_neg_int_is_zero__extern(int *target) { return ck_pr_neg_int_is_zero(target); }
void ck_pr_neg_uint__extern(unsigned int *target) { ck_pr_neg_uint(target); }
bool ck_pr_neg_uint_is_zero__extern(unsigned int *target) { return ck_pr_neg_uint_is_zero(target); }
void ck_pr_neg_64__extern(uint64_t *target) { ck_pr_neg_64(target); }
bool ck_pr_neg_64_is_zero__extern(uint64_t *target) { return ck_pr_neg_64_is_zero(target); }
void ck_pr_neg_32__extern(uint32_t *target) { ck_pr_neg_32(target); }
bool ck_pr_neg_32_is_zero__extern(uint32_t *target) { return ck_pr_neg_32_is_zero(target); }
void ck_pr_neg_16__extern(uint16_t *target) { ck_pr_neg_16(target); }
bool ck_pr_neg_16_is_zero__extern(uint16_t *target) { return ck_pr_neg_16_is_zero(target); }
void ck_pr_neg_8__extern(uint8_t *target) { ck_pr_neg_8(target); }
bool ck_pr_neg_8_is_zero__extern(uint8_t *target) { return ck_pr_neg_8_is_zero(target); }
void ck_pr_not_ptr__extern(void *target) { ck_pr_not_ptr(target); }
void ck_pr_not_char__extern(char *target) { ck_pr_not_char(target); }
void ck_pr_not_int__extern(int *target) { ck_pr_not_int(target); }
void ck_pr_not_uint__extern(unsigned int *target) { ck_pr_not_uint(target); }
void ck_pr_not_64__extern(uint64_t *target) { ck_pr_not_64(target); }
void ck_pr_not_32__extern(uint32_t *target) { ck_pr_not_32(target); }
void ck_pr_not_16__extern(uint16_t *target) { ck_pr_not_16(target); }
void ck_pr_not_8__extern(uint8_t *target) { ck_pr_not_8(target); }
void ck_pr_add_ptr__extern(void *target, uintptr_t d) { ck_pr_add_ptr(target, d); }
void ck_pr_add_char__extern(char *target, char d) { ck_pr_add_char(target, d); }
void ck_pr_add_int__extern(int *target, int d) { ck_pr_add_int(target, d); }
void ck_pr_add_uint__extern(unsigned int *target, unsigned int d) { ck_pr_add_uint(target, d); }
void ck_pr_add_64__extern(uint64_t *target, uint64_t d) { ck_pr_add_64(target, d); }
void ck_pr_add_32__extern(uint32_t *target, uint32_t d) { ck_pr_add_32(target, d); }
void ck_pr_add_16__extern(uint16_t *target, uint16_t d) { ck_pr_add_16(target, d); }
void ck_pr_add_8__extern(uint8_t *target, uint8_t d) { ck_pr_add_8(target, d); }
void ck_pr_sub_ptr__extern(void *target, uintptr_t d) { ck_pr_sub_ptr(target, d); }
void ck_pr_sub_char__extern(char *target, char d) { ck_pr_sub_char(target, d); }
void ck_pr_sub_int__extern(int *target, int d) { ck_pr_sub_int(target, d); }
void ck_pr_sub_uint__extern(unsigned int *target, unsigned int d) { ck_pr_sub_uint(target, d); }
void ck_pr_sub_64__extern(uint64_t *target, uint64_t d) { ck_pr_sub_64(target, d); }
void ck_pr_sub_32__extern(uint32_t *target, uint32_t d) { ck_pr_sub_32(target, d); }
void ck_pr_sub_16__extern(uint16_t *target, uint16_t d) { ck_pr_sub_16(target, d); }
void ck_pr_sub_8__extern(uint8_t *target, uint8_t d) { ck_pr_sub_8(target, d); }
void ck_pr_and_ptr__extern(void *target, uintptr_t d) { ck_pr_and_ptr(target, d); }
void ck_pr_and_char__extern(char *target, char d) { ck_pr_and_char(target, d); }
void ck_pr_and_int__extern(int *target, int d) { ck_pr_and_int(target, d); }
void ck_pr_and_uint__extern(unsigned int *target, unsigned int d) { ck_pr_and_uint(target, d); }
void ck_pr_and_64__extern(uint64_t *target, uint64_t d) { ck_pr_and_64(target, d); }
void ck_pr_and_32__extern(uint32_t *target, uint32_t d) { ck_pr_and_32(target, d); }
void ck_pr_and_16__extern(uint16_t *target, uint16_t d) { ck_pr_and_16(target, d); }
void ck_pr_and_8__extern(uint8_t *target, uint8_t d) { ck_pr_and_8(target, d); }
void ck_pr_or_ptr__extern(void *target, uintptr_t d) { ck_pr_or_ptr(target, d); }
void ck_pr_or_char__extern(char *target, char d) { ck_pr_or_char(target, d); }
void ck_pr_or_int__extern(int *target, int d) { ck_pr_or_int(target, d); }
void ck_pr_or_uint__extern(unsigned int *target, unsigned int d) { ck_pr_or_uint(target, d); }
void ck_pr_or_64__extern(uint64_t *target, uint64_t d) { ck_pr_or_64(target, d); }
void ck_pr_or_32__extern(uint32_t *target, uint32_t d) { ck_pr_or_32(target, d); }
void ck_pr_or_16__extern(uint16_t *target, uint16_t d) { ck_pr_or_16(target, d); }
void ck_pr_or_8__extern(uint8_t *target, uint8_t d) { ck_pr_or_8(target, d); }
void ck_pr_xor_ptr__extern(void *target, uintptr_t d) { ck_pr_xor_ptr(target, d); }
void ck_pr_xor_char__extern(char *target, char d) { ck_pr_xor_char(target, d); }
void ck_pr_xor_int__extern(int *target, int d) { ck_pr_xor_int(target, d); }
void ck_pr_xor_uint__extern(unsigned int *target, unsigned int d) { ck_pr_xor_uint(target, d); }
void ck_pr_xor_64__extern(uint64_t *target, uint64_t d) { ck_pr_xor_64(target, d); }
void ck_pr_xor_32__extern(uint32_t *target, uint32_t d) { ck_pr_xor_32(target, d); }
void ck_pr_xor_16__extern(uint16_t *target, uint16_t d) { ck_pr_xor_16(target, d); }
void ck_pr_xor_8__extern(uint8_t *target, uint8_t d) { ck_pr_xor_8(target, d); }
bool ck_pr_cas_ptr__extern(void *target, void *compare, void *set) { return ck_pr_cas_ptr(target, compare, set); }
bool ck_pr_cas_ptr_value__extern(void *target, void *compare, void *set, void *v) { return ck_pr_cas_ptr_value(target, compare, set, v); }
bool ck_pr_cas_char__extern(char *target, char compare, char set) { return ck_pr_cas_char(target, compare, set); }
bool ck_pr_cas_char_value__extern(char *target, char compare, char set, char *v) { return ck_pr_cas_char_value(target, compare, set, v); }
bool ck_pr_cas_int__extern(int *target, int compare, int set) { return ck_pr_cas_int(target, compare, set); }
bool ck_pr_cas_int_value__extern(int *target, int compare, int set, int *v) { return ck_pr_cas_int_value(target, compare, set, v); }
bool ck_pr_cas_uint__extern(unsigned int *target, unsigned int compare, unsigned int set) { return ck_pr_cas_uint(target, compare, set); }
bool ck_pr_cas_uint_value__extern(unsigned int *target, unsigned int compare, unsigned int set, unsigned int *v) { return ck_pr_cas_uint_value(target, compare, set, v); }
bool ck_pr_cas_double__extern(double *target, double compare, double set) { return ck_pr_cas_double(target, compare, set); }
bool ck_pr_cas_double_value__extern(double *target, double compare, double set, double *v) { return ck_pr_cas_double_value(target, compare, set, v); }
bool ck_pr_cas_64__extern(uint64_t *target, uint64_t compare, uint64_t set) { return ck_pr_cas_64(target, compare, set); }
bool ck_pr_cas_64_value__extern(uint64_t *target, uint64_t compare, uint64_t set, uint64_t *v) { return ck_pr_cas_64_value(target, compare, set, v); }
bool ck_pr_cas_32__extern(uint32_t *target, uint32_t compare, uint32_t set) { return ck_pr_cas_32(target, compare, set); }
bool ck_pr_cas_32_value__extern(uint32_t *target, uint32_t compare, uint32_t set, uint32_t *v) { return ck_pr_cas_32_value(target, compare, set, v); }
bool ck_pr_cas_16__extern(uint16_t *target, uint16_t compare, uint16_t set) { return ck_pr_cas_16(target, compare, set); }
bool ck_pr_cas_16_value__extern(uint16_t *target, uint16_t compare, uint16_t set, uint16_t *v) { return ck_pr_cas_16_value(target, compare, set, v); }
bool ck_pr_cas_8__extern(uint8_t *target, uint8_t compare, uint8_t set) { return ck_pr_cas_8(target, compare, set); }
bool ck_pr_cas_8_value__extern(uint8_t *target, uint8_t compare, uint8_t set, uint8_t *v) { return ck_pr_cas_8_value(target, compare, set, v); }
bool ck_pr_cas_64_2__extern(uint64_t target [2], uint64_t compare [2], uint64_t set [2]) { return ck_pr_cas_64_2(target, compare, set); }
bool ck_pr_cas_ptr_2__extern(void *t, void *c, void *s) { return ck_pr_cas_ptr_2(t, c, s); }
bool ck_pr_cas_64_2_value__extern(uint64_t target [2], uint64_t compare [2], uint64_t set [2], uint64_t v [2]) { return ck_pr_cas_64_2_value(target, compare, set, v); }
bool ck_pr_cas_ptr_2_value__extern(void *t, void *c, void *s, void *v) { return ck_pr_cas_ptr_2_value(t, c, s, v); }
bool ck_pr_cas_double_2__extern(double t [2], double c [2], double s [2]) { return ck_pr_cas_double_2(t, c, s); }
bool ck_pr_cas_double_2_value__extern(double *t, double c [2], double s [2], double *v) { return ck_pr_cas_double_2_value(t, c, s, v); }
bool ck_pr_cas_char_16__extern(char t [16], char c [16], char s [16]) { return ck_pr_cas_char_16(t, c, s); }
bool ck_pr_cas_char_16_value__extern(char *t, char c [16], char s [16], char *v) { return ck_pr_cas_char_16_value(t, c, s, v); }
bool ck_pr_cas_int_4__extern(int t [4], int c [4], int s [4]) { return ck_pr_cas_int_4(t, c, s); }
bool ck_pr_cas_int_4_value__extern(int *t, int c [4], int s [4], int *v) { return ck_pr_cas_int_4_value(t, c, s, v); }
bool ck_pr_cas_uint_4__extern(unsigned int t [4], unsigned int c [4], unsigned int s [4]) { return ck_pr_cas_uint_4(t, c, s); }
bool ck_pr_cas_uint_4_value__extern(unsigned int *t, unsigned int c [4], unsigned int s [4], unsigned int *v) { return ck_pr_cas_uint_4_value(t, c, s, v); }
bool ck_pr_cas_32_4__extern(uint32_t t [4], uint32_t c [4], uint32_t s [4]) { return ck_pr_cas_32_4(t, c, s); }
bool ck_pr_cas_32_4_value__extern(uint32_t *t, uint32_t c [4], uint32_t s [4], uint32_t *v) { return ck_pr_cas_32_4_value(t, c, s, v); }
bool ck_pr_cas_16_8__extern(uint16_t t [8], uint16_t c [8], uint16_t s [8]) { return ck_pr_cas_16_8(t, c, s); }
bool ck_pr_cas_16_8_value__extern(uint16_t *t, uint16_t c [8], uint16_t s [8], uint16_t *v) { return ck_pr_cas_16_8_value(t, c, s, v); }
bool ck_pr_cas_8_16__extern(uint8_t t [16], uint8_t c [16], uint8_t s [16]) { return ck_pr_cas_8_16(t, c, s); }
bool ck_pr_cas_8_16_value__extern(uint8_t *t, uint8_t c [16], uint8_t s [16], uint8_t *v) { return ck_pr_cas_8_16_value(t, c, s, v); }
bool ck_pr_btc_ptr__extern(void *target, unsigned int b) { return ck_pr_btc_ptr(target, b); }
bool ck_pr_btc_uint__extern(unsigned int *target, unsigned int b) { return ck_pr_btc_uint(target, b); }
bool ck_pr_btc_int__extern(int *target, unsigned int b) { return ck_pr_btc_int(target, b); }
bool ck_pr_btc_64__extern(uint64_t *target, unsigned int b) { return ck_pr_btc_64(target, b); }
bool ck_pr_btc_32__extern(uint32_t *target, unsigned int b) { return ck_pr_btc_32(target, b); }
bool ck_pr_btc_16__extern(uint16_t *target, unsigned int b) { return ck_pr_btc_16(target, b); }
bool ck_pr_bts_ptr__extern(void *target, unsigned int b) { return ck_pr_bts_ptr(target, b); }
bool ck_pr_bts_uint__extern(unsigned int *target, unsigned int b) { return ck_pr_bts_uint(target, b); }
bool ck_pr_bts_int__extern(int *target, unsigned int b) { return ck_pr_bts_int(target, b); }
bool ck_pr_bts_64__extern(uint64_t *target, unsigned int b) { return ck_pr_bts_64(target, b); }
bool ck_pr_bts_32__extern(uint32_t *target, unsigned int b) { return ck_pr_bts_32(target, b); }
bool ck_pr_bts_16__extern(uint16_t *target, unsigned int b) { return ck_pr_bts_16(target, b); }
bool ck_pr_btr_ptr__extern(void *target, unsigned int b) { return ck_pr_btr_ptr(target, b); }
bool ck_pr_btr_uint__extern(unsigned int *target, unsigned int b) { return ck_pr_btr_uint(target, b); }
bool ck_pr_btr_int__extern(int *target, unsigned int b) { return ck_pr_btr_int(target, b); }
bool ck_pr_btr_64__extern(uint64_t *target, unsigned int b) { return ck_pr_btr_64(target, b); }
bool ck_pr_btr_32__extern(uint32_t *target, unsigned int b) { return ck_pr_btr_32(target, b); }
bool ck_pr_btr_16__extern(uint16_t *target, unsigned int b) { return ck_pr_btr_16(target, b); }
void ck_pr_barrier__extern(void) { ck_pr_barrier(); }
void ck_pr_fence_load_depends__extern(void) { ck_pr_fence_load_depends(); }
void ck_pr_fence_atomic__extern(void) { ck_pr_fence_atomic(); }
void ck_pr_fence_atomic_load__extern(void) { ck_pr_fence_atomic_load(); }
void ck_pr_fence_atomic_store__extern(void) { ck_pr_fence_atomic_store(); }
void ck_pr_fence_store_atomic__extern(void) { ck_pr_fence_store_atomic(); }
void ck_pr_fence_load_atomic__extern(void) { ck_pr_fence_load_atomic(); }
void ck_pr_fence_load_store__extern(void) { ck_pr_fence_load_store(); }
void ck_pr_fence_store_load__extern(void) { ck_pr_fence_store_load(); }
void ck_pr_fence_load__extern(void) { ck_pr_fence_load(); }
void ck_pr_fence_store__extern(void) { ck_pr_fence_store(); }
void ck_pr_fence_memory__extern(void) { ck_pr_fence_memory(); }
void ck_pr_fence_acquire__extern(void) { ck_pr_fence_acquire(); }
void ck_pr_fence_release__extern(void) { ck_pr_fence_release(); }
void ck_pr_fence_acqrel__extern(void) { ck_pr_fence_acqrel(); }
void ck_pr_fence_lock__extern(void) { ck_pr_fence_lock(); }
void ck_pr_fence_unlock__extern(void) { ck_pr_fence_unlock(); }
void ck_pr_add_double__extern(double *target, double value) { ck_pr_add_double(target, value); }
void ck_pr_sub_double__extern(double *target, double value) { ck_pr_sub_double(target, value); }
void ck_pr_inc_char_zero__extern(char *target, bool *zero) { ck_pr_inc_char_zero(target, zero); }
void ck_pr_dec_char_zero__extern(char *target, bool *zero) { ck_pr_dec_char_zero(target, zero); }
void ck_pr_inc_int_zero__extern(int *target, bool *zero) { ck_pr_inc_int_zero(target, zero); }
void ck_pr_dec_int_zero__extern(int *target, bool *zero) { ck_pr_dec_int_zero(target, zero); }
void ck_pr_inc_double__extern(double *target) { ck_pr_inc_double(target); }
void ck_pr_dec_double__extern(double *target) { ck_pr_dec_double(target); }
void ck_pr_inc_uint_zero__extern(unsigned int *target, bool *zero) { ck_pr_inc_uint_zero(target, zero); }
void ck_pr_dec_uint_zero__extern(unsigned int *target, bool *zero) { ck_pr_dec_uint_zero(target, zero); }
void ck_pr_inc_ptr_zero__extern(void *target, bool *zero) { ck_pr_inc_ptr_zero(target, zero); }
void ck_pr_dec_ptr_zero__extern(void *target, bool *zero) { ck_pr_dec_ptr_zero(target, zero); }
void ck_pr_inc_64_zero__extern(uint64_t *target, bool *zero) { ck_pr_inc_64_zero(target, zero); }
void ck_pr_dec_64_zero__extern(uint64_t *target, bool *zero) { ck_pr_dec_64_zero(target, zero); }
void ck_pr_inc_32_zero__extern(uint32_t *target, bool *zero) { ck_pr_inc_32_zero(target, zero); }
void ck_pr_dec_32_zero__extern(uint32_t *target, bool *zero) { ck_pr_dec_32_zero(target, zero); }
void ck_pr_inc_16_zero__extern(uint16_t *target, bool *zero) { ck_pr_inc_16_zero(target, zero); }
void ck_pr_dec_16_zero__extern(uint16_t *target, bool *zero) { ck_pr_dec_16_zero(target, zero); }
void ck_pr_inc_8_zero__extern(uint8_t *target, bool *zero) { ck_pr_inc_8_zero(target, zero); }
void ck_pr_dec_8_zero__extern(uint8_t *target, bool *zero) { ck_pr_dec_8_zero(target, zero); }
void ck_pr_neg_double__extern(double *target) { ck_pr_neg_double(target); }
double ck_pr_faa_double__extern(double *target, double delta) { return ck_pr_faa_double(target, delta); }
unsigned int ck_ring_size__extern(const struct ck_ring *ring) { return ck_ring_size(ring); }
unsigned int ck_ring_capacity__extern(const struct ck_ring *ring) { return ck_ring_capacity(ring); }
bool ck_ring_repair__extern(struct ck_ring *ring) { return ck_ring_repair(ring); }
bool ck_ring_valid__extern(const struct ck_ring *ring) { return ck_ring_valid(ring); }
void ck_ring_init__extern(struct ck_ring *ring, unsigned int size) { ck_ring_init(ring, size); }
bool ck_ring_enqueue_spsc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry, unsigned int *size) { return ck_ring_enqueue_spsc_size(ring, buffer, entry, size); }
bool ck_ring_enqueue_spsc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry) { return ck_ring_enqueue_spsc(ring, buffer, entry); }
void * ck_ring_enqueue_reserve_spsc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *size) { return ck_ring_enqueue_reserve_spsc_size(ring, buffer, size); }
void * ck_ring_enqueue_reserve_spsc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer) { return ck_ring_enqueue_reserve_spsc(ring, buffer); }
void ck_ring_enqueue_commit_spsc__extern(struct ck_ring *ring) { ck_ring_enqueue_commit_spsc(ring); }
bool ck_ring_dequeue_spsc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_dequeue_spsc(ring, buffer, data); }
bool ck_ring_enqueue_mpmc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry) { return ck_ring_enqueue_mpmc(ring, buffer, entry); }
bool ck_ring_enqueue_mpmc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry, unsigned int *size) { return ck_ring_enqueue_mpmc_size(ring, buffer, entry, size); }
void * ck_ring_enqueue_reserve_mpmc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *ticket) { return ck_ring_enqueue_reserve_mpmc(ring, buffer, ticket); }
void * ck_ring_enqueue_reserve_mpmc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *ticket, unsigned int *size) { return ck_ring_enqueue_reserve_mpmc_size(ring, buffer, ticket, size); }
void ck_ring_enqueue_commit_mpmc__extern(struct ck_ring *ring, unsigned int ticket) { ck_ring_enqueue_commit_mpmc(ring, ticket); }
bool ck_ring_trydequeue_mpmc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_trydequeue_mpmc(ring, buffer, data); }
bool ck_ring_dequeue_mpmc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_dequeue_mpmc(ring, buffer, data); }
void * ck_ring_enqueue_reserve_spmc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *size) { return ck_ring_enqueue_reserve_spmc_size(ring, buffer, size); }
void * ck_ring_enqueue_reserve_spmc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer) { return ck_ring_enqueue_reserve_spmc(ring, buffer); }
void ck_ring_enqueue_commit_spmc__extern(struct ck_ring *ring) { ck_ring_enqueue_commit_spmc(ring); }
bool ck_ring_enqueue_spmc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry, unsigned int *size) { return ck_ring_enqueue_spmc_size(ring, buffer, entry, size); }
bool ck_ring_enqueue_spmc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry) { return ck_ring_enqueue_spmc(ring, buffer, entry); }
bool ck_ring_trydequeue_spmc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_trydequeue_spmc(ring, buffer, data); }
bool ck_ring_dequeue_spmc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_dequeue_spmc(ring, buffer, data); }
void * ck_ring_enqueue_reserve_mpsc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *ticket) { return ck_ring_enqueue_reserve_mpsc(ring, buffer, ticket); }
void * ck_ring_enqueue_reserve_mpsc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, unsigned int *ticket, unsigned int *size) { return ck_ring_enqueue_reserve_mpsc_size(ring, buffer, ticket, size); }
void ck_ring_enqueue_commit_mpsc__extern(struct ck_ring *ring, unsigned int ticket) { ck_ring_enqueue_commit_mpsc(ring, ticket); }
bool ck_ring_enqueue_mpsc__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry) { return ck_ring_enqueue_mpsc(ring, buffer, entry); }
bool ck_ring_enqueue_mpsc_size__extern(struct ck_ring *ring, struct ck_ring_buffer *buffer, const void *entry, unsigned int *size) { return ck_ring_enqueue_mpsc_size(ring, buffer, entry, size); }
bool ck_ring_dequeue_mpsc__extern(struct ck_ring *ring, const struct ck_ring_buffer *buffer, void *data) { return ck_ring_dequeue_mpsc(ring, buffer, data); }
