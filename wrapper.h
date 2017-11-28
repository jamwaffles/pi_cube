#include <stdint.h>
#include <string.h>

#include <ws2811.h>

ws2811_t ledstring = {
	.freq = 800000,
	.dmanum = 5,
	.channel = {
		[0] = {
			.gpionum = 18,
			.count = 256,
			.invert = 0,
			.brightness = 32,
		},
		[1] = {
			.gpionum = 0,
			.count = 0,
			.invert = 0,
			.brightness = 0,
		},
	},
};

void ws2811_set_led(ws2811_t *ws2811, int index, uint32_t value) {
	ws2811->channel[0].leds[index] = value;
}

void ws2811_clear(ws2811_t *ws2811) {
	for (int chan = 0; chan < RPI_PWM_CHANNELS; chan++) {
		ws2811_channel_t *channel = &ws2811->channel[chan];
		memset(channel->leds, 0, sizeof(ws2811_led_t) * channel->count);
	}
}

void ws2811_set_bitmap(ws2811_t *ws2811, void* a, int len) {
	memcpy(ws2811->channel[0].leds, a, len);
}
