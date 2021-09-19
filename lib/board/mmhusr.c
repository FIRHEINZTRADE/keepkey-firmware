/*
 * This file is part of the KEEPKEY project
 *
 * Copyright (C) 2018 KEEPKEY
 *
 * This library is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef EMULATOR
#include <libopencm3/stm32/flash.h>
#include <libopencm3/stm32/timer.h>
#else
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#endif

#include "keepkey/board/supervise.h"
#include "keepkey/board/keepkey_board.h"
#include "keepkey/board/keepkey_display.h"

#include <string.h>
#include <stdint.h>
#include <stdio.h>

void __attribute__((noreturn)) mmhisr(void) {
  shutdown_with_error(SHUTDOWN_ERROR_MEMFAULT);
}
