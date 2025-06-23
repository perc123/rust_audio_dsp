use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Q_BUTTERWORTH_F32, Type};

#[repr(C)]
pub struct EQFilter {
    filter: DirectForm1<f32>,
}

#[no_mangle]
pub extern "C" fn eq_filter_create(
    sample_rate: f32,
    cutoff_hz: f32,
    gain_db: f32
) -> *mut EQFilter {
    let coeffs = Coefficients::<f32>::from_params(
        Type::LowShelf(gain_db),     // pass gain here
        sample_rate.hz(),
        cutoff_hz.hz(),
        Q_BUTTERWORTH_F32
    ).unwrap();

    let filter = DirectForm1::<f32>::new(coeffs);

    let eq = EQFilter { filter };

    Box::into_raw(Box::new(eq))
}

#[no_mangle]
pub extern "C" fn eq_filter_process(
    eq_ptr: *mut EQFilter,
    input: *const f32,
    output: *mut f32,
    nframes: usize
) {
    unsafe {
        let eq = &mut *eq_ptr;
        let in_slice = std::slice::from_raw_parts(input, nframes);
        let out_slice = std::slice::from_raw_parts_mut(output, nframes);

        for i in 0..nframes {
            out_slice[i] = eq.filter.run(in_slice[i]);
        }
    }
}

#[no_mangle]
pub extern "C" fn eq_filter_destroy(eq_ptr: *mut EQFilter) {
    if !eq_ptr.is_null() {
        unsafe {
            Box::from_raw(eq_ptr);
        }
    }
}
