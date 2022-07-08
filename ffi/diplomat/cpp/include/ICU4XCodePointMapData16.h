#ifndef ICU4XCodePointMapData16_H
#define ICU4XCodePointMapData16_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XCodePointMapData16 ICU4XCodePointMapData16;
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XCodePointMapData16_ICU4XError.h"

diplomat_result_box_ICU4XCodePointMapData16_ICU4XError ICU4XCodePointMapData16_try_get_script(const ICU4XDataProvider* provider);

uint16_t ICU4XCodePointMapData16_get(const ICU4XCodePointMapData16* self, char32_t cp);
void ICU4XCodePointMapData16_destroy(ICU4XCodePointMapData16* self);

#ifdef __cplusplus
}
#endif
#endif