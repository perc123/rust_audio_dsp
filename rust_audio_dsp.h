// rust_audio_dsp.h

#ifdef __cplusplus
extern "C" {
#endif

typedef struct EQFilter EQFilter;

EQFilter* eq_filter_create(float sample_rate, float cutoff_hz, float gain_db);
void eq_filter_process(EQFilter* eq, const float* input, float* output, size_t nframes);
void eq_filter_destroy(EQFilter* eq);

#ifdef __cplusplus
}
#endif
