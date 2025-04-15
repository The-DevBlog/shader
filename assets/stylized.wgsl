#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// Scene color texture and sampler.
@group(0) @binding(0)
var sceneTexture : texture_2d<f32>;

@group(0) @binding(1)
var sceneSampler : sampler;

// Depth texture is declared as a depth texture.
@group(0) @binding(3)
var depthTexture : texture_depth_2d;

// Normal texture (assumed to store normals as RGB values).
@group(0) @binding(4)
var normalTexture : texture_2d<f32>;

// A helper function to compute luminance.
fn luminance(col: vec4<f32>) -> f32 {
    return dot(col.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
}

@fragment
fn fragment(input: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // SETTINGS for the stylized effect
    let colorSteps: f32 = 60.0;               // Number of quantization steps for posterization
    let edgeThreshold: f32 = 0.001;            // Base threshold for luminance (Sobel) edge detection
    let edgeColor: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Outline color (red here)
    let normalEdgeThreshold: f32 = 1.0;      // Base threshold for normal difference edge detection
    let depthEdgeThreshold: f32 = 1.0;       // Base threshold for depth difference edge detection
    let lineThickness: f32 = 1.0;


    // Fetch the center pixel's color.
    let centerColor = textureSample(sceneTexture, sceneSampler, input.uv);
    // Define a pixel's size in UV space (based on a target resolution; adjust if needed).
    let pixelSize = vec2<f32>(1.0 / 1920.0, 1.0 / 1080.0);
    // Multiply by lineThickness to adjust the sampling radius.
    let adjustedPixelSize = pixelSize * lineThickness;

    //
    // Luminance Edge Detection (using a Sobel filter)
    //
    let c0 = textureSample(sceneTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, -adjustedPixelSize.y));
    let c1 = textureSample(sceneTexture, sceneSampler, input.uv + vec2(0.0, -adjustedPixelSize.y));
    let c2 = textureSample(sceneTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, -adjustedPixelSize.y));
    let c3 = textureSample(sceneTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, 0.0));
    let c5 = textureSample(sceneTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, 0.0));
    let c6 = textureSample(sceneTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, adjustedPixelSize.y));
    let c7 = textureSample(sceneTexture, sceneSampler, input.uv + vec2(0.0, adjustedPixelSize.y));
    let c8 = textureSample(sceneTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, adjustedPixelSize.y));

    let lum0 = luminance(c0);
    let lum1 = luminance(c1);
    let lum2 = luminance(c2);
    let lum3 = luminance(c3);
    let lum4 = luminance(centerColor);
    let lum5 = luminance(c5);
    let lum6 = luminance(c6);
    let lum7 = luminance(c7);
    let lum8 = luminance(c8);

    let gx = (lum2 + 2.0 * lum5 + lum8) - (lum0 + 2.0 * lum3 + lum6);
    let gy = (lum6 + 2.0 * lum7 + lum8) - (lum0 + 2.0 * lum1 + lum2);
    let edgeMagnitude = sqrt(gx * gx + gy * gy);
    // Smooth the luminance edge detection rather than a hard threshold.
    let lumEdge = smoothstep(edgeThreshold, edgeThreshold + 0.05, edgeMagnitude);

    //
    // Normal Edge Detection
    //
    let centerNormal = textureSample(normalTexture, sceneSampler, input.uv).rgb;
    let n0 = textureSample(normalTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, -adjustedPixelSize.y)).rgb;
    let n1 = textureSample(normalTexture, sceneSampler, input.uv + vec2(             0.0, -adjustedPixelSize.y)).rgb;
    let n2 = textureSample(normalTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, -adjustedPixelSize.y)).rgb;
    let n3 = textureSample(normalTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, 0.0)).rgb;
    let n5 = textureSample(normalTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, 0.0)).rgb;
    let n6 = textureSample(normalTexture, sceneSampler, input.uv + vec2(-adjustedPixelSize.x, adjustedPixelSize.y)).rgb;
    let n7 = textureSample(normalTexture, sceneSampler, input.uv + vec2(             0.0, adjustedPixelSize.y)).rgb;
    let n8 = textureSample(normalTexture, sceneSampler, input.uv + vec2( adjustedPixelSize.x, adjustedPixelSize.y)).rgb;

    let dot0 = dot(centerNormal, n0);
    let dot1 = dot(centerNormal, n1);
    let dot2 = dot(centerNormal, n2);
    let dot3 = dot(centerNormal, n3);
    let dot5 = dot(centerNormal, n5);
    let dot6 = dot(centerNormal, n6);
    let dot7 = dot(centerNormal, n7);
    let dot8 = dot(centerNormal, n8);
    let normalDiff = max(
                        max(1.0 - dot0, 1.0 - dot1),
                        max(max(1.0 - dot2, 1.0 - dot3),
                            max(1.0 - dot5, max(1.0 - dot6, max(1.0 - dot7, 1.0 - dot8))))
                      );
    let normalEdge = smoothstep(normalEdgeThreshold, normalEdgeThreshold + 15.0, normalDiff);

    //
    // Depth Edge Detection using textureLoad
    //
    // For depth, we must convert the UV to pixel coordinates.
    let resolution = vec2<f32>(1920.0, 1080.0); // Target resolution (adjust as needed)
    // Get the center pixel coordinate (in integer pixels)
    let centerCoord = vec2<i32>(floor(input.uv * resolution));
    // Compute the offset for depth detection in pixel units.
    // Since adjustedPixelSize in UV multiplied by resolution yields the pixel offset:
    let depthOffset = vec2<i32>(
        i32(round(adjustedPixelSize.x * resolution.x)),
        i32(round(adjustedPixelSize.y * resolution.y))
    );

    // Sample depth from neighboring pixels using textureLoad.
    let centerDepth = textureLoad(depthTexture, centerCoord, 0);
    let d0 = textureLoad(depthTexture, centerCoord + vec2<i32>(-depthOffset.x, -depthOffset.y), 0);
    let d1 = textureLoad(depthTexture, centerCoord + vec2<i32>( 0, -depthOffset.y), 0);
    let d2 = textureLoad(depthTexture, centerCoord + vec2<i32>( depthOffset.x, -depthOffset.y), 0);
    let d3 = textureLoad(depthTexture, centerCoord + vec2<i32>(-depthOffset.x,  0), 0);
    let d5 = textureLoad(depthTexture, centerCoord + vec2<i32>( depthOffset.x,  0), 0);
    let d6 = textureLoad(depthTexture, centerCoord + vec2<i32>(-depthOffset.x, depthOffset.y), 0);
    let d7 = textureLoad(depthTexture, centerCoord + vec2<i32>( 0, depthOffset.y), 0);
    let d8 = textureLoad(depthTexture, centerCoord + vec2<i32>( depthOffset.x, depthOffset.y), 0);
    let depthDiff = max(
                        max(abs(centerDepth - d0), abs(centerDepth - d1)),
                        max(
                            max(abs(centerDepth - d2), abs(centerDepth - d3)),
                            max(abs(centerDepth - d5),
                                max(abs(centerDepth - d6),
                                    max(abs(centerDepth - d7), abs(centerDepth - d8)))
                            )
                        )
                     );
    let depthEdge = smoothstep(depthEdgeThreshold, depthEdgeThreshold + 5.0, depthDiff);

    //
    // Combine all edge detection results into a smooth composite edge mask.
    //
    let compositeEdge = max(lumEdge, max(normalEdge, depthEdge));
    let edgeStrength: f32 = 5.0; // Adjust this value as needed.
    let boostedEdge = clamp(compositeEdge * edgeStrength, 0.0, 1.0);

    //
    // Posterization: Quantize the center color.
    //
    let quantized = floor(centerColor.rgb * colorSteps + 0.5) / colorSteps;
    let baseColor = vec4<f32>(quantized, 1.0);

    // Blend in the outline (edge) color using the composite edge mask.
    // let finalColor = mix(baseColor, edgeColor, compositeEdge);
    let finalColor = mix(baseColor, edgeColor, boostedEdge);
    return finalColor;
}
