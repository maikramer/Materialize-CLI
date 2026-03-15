// src/shaders/normal.wgsl

@group(0) @binding(0)
var height_texture: texture_2d<f32>;

@group(0) @binding(1)
var output_texture: texture_storage_2d<rgba8unorm, write>;

// Sobel operator kernels
const SOBEL_X: array<f32, 9> = array<f32, 9>(-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0);
const SOBEL_Y: array<f32, 9> = array<f32, 9>(-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0);

fn sample_height(coords: vec2<i32>, dims: vec2<u32>) -> f32 {
    let clamped = clamp(coords, vec2<i32>(0), vec2<i32>(dims) - vec2<i32>(1));
    return textureLoad(height_texture, clamped, 0).r;
}

fn sobel_gradient(center: vec2<i32>, dims: vec2<u32>) -> vec2<f32> {
    var gx = 0.0;
    var gy = 0.0;

    var idx = 0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let sample_coords = center + vec2<i32>(x, y);
            let h = sample_height(sample_coords, dims);
            gx += h * SOBEL_X[idx];
            gy += h * SOBEL_Y[idx];
            idx += 1;
        }
    }

    return vec2<f32>(gx, gy);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(height_texture);
    let coords = vec2<i32>(global_id.xy);

    if (coords.x >= i32(dims.x) || coords.y >= i32(dims.y)) {
        return;
    }

    // Calculate gradient using Sobel
    let gradient = sobel_gradient(coords, dims);

    // Scale gradient for intensity
    let scale = 2.0;
    let gx = gradient.x * scale;
    let gy = gradient.y * scale;

    // Construct normal vector (pointing up, against gradient)
    var normal = vec3<f32>(-gx, -gy, 1.0);
    normal = normalize(normal);

    // Encode to [0, 1] range for RGB8 storage
    let encoded = normal * 0.5 + 0.5;

    textureStore(output_texture, coords, vec4<f32>(encoded, 1.0));
}
