# TODO

标记意味着已实现 `x` 或不打算实现 `xx`

- [xx] -h, --help                         show this help message and exit
- [x] -M, --mode [MODEL]                 run mode (txt2img or img2img or convert, default: txt2img)
- [x] -t, --threads N                    number of threads to use during computation (default: -1)
                                     If threads <= 0, then threads will be set to the number of CPU physical cores
- [x] -m, --model [MODEL]                path to full model
- [xx] --diffusion-model                  path to the standalone diffusion model
- [x] --clip_l                           path to the clip-l text encoder
- [x] --clip_g                           path to the clip-g text encoder
- [x] --t5xxl                            path to the the t5xxl text encoder
- [x] --vae [VAE]                        path to vae
- [x] --taesd [TAESD_PATH]               path to taesd. Using Tiny AutoEncoder for fast decoding (low quality)
- [x] --control-net [CONTROL_PATH]       path to control net model
- [x] --embd-dir [EMBEDDING_PATH]        path to embeddings
- [x] --stacked-id-embd-dir [DIR]        path to PHOTOMAKER stacked id embeddings
- [x] --input-id-images-dir [DIR]        path to PHOTOMAKER input id images dir
- [x] --normalize-input                  normalize PHOTOMAKER input id images
- [x] --upscale-model [ESRGAN_PATH]      path to esrgan model. Upscale images after generate, just RealESRGAN_x4plus_anime_6B supported by now
- [x] --upscale-repeats                  Run the ESRGAN upscaler this many times (default 1)
- [x] --type [TYPE]                      weight type (examples: f32, f16, q4_0, q4_1, q5_0, q5_1, q8_0, q2_K, q3_K, q4_K)            If not specified, the default is the type of the weight file
- [x] --lora-model-dir [DIR]             lora model directory
- [x] -i, --init-img [IMAGE]             path to the input image, required by img2img
- [x] --mask [MASK]                      path to the mask image, required by img2img with mask
- [] --control-image [IMAGE]            path to image condition, control net
- [] -o, --output OUTPUT                path to write result image to (default: ./output.png)
- [] -p, --prompt [PROMPT]              the prompt to render
- [] -n, --negative-prompt PROMPT       the negative prompt (default: "")
- [] --cfg-scale SCALE                  unconditional guidance scale: (default: 7.0)
- [] --guidance SCALE                   guidance scale for img2img (default: 3.5)
- [] --slg-scale SCALE                  skip layer guidance (SLG) scale, only for DiT models: (default: 0)
                                     0 means disabled, a value of 2.5 is nice for sd3.5 medium
- [] --skip-layers LAYERS               Layers to skip for SLG steps: (default: [7,8,9])
- [] --skip-layer-start START           SLG enabling point: (default: 0.01)
- [] --skip-layer-end END               SLG disabling point: (default: 0.2)
                                     SLG will be enabled at step int([STEPS]*[START]) and disabled at int([STEPS]*[END])
- [] --strength STRENGTH                strength for noising/unnoising (default: 0.75)
- [] --style-ratio STYLE-RATIO          strength for keeping input identity (default: 20%)
- [] --control-strength STRENGTH        strength to apply Control Net (default: 0.9)
                                     1.0 corresponds to full destruction of information in init image
- [] -H, --height H                     image height, in pixel space (default: 512)
- [] -W, --width W                      image width, in pixel space (default: 512)
- [] --sampling-method {euler, euler_a, heun, dpm2, dpm++2s_a, dpm++2m, dpm++2mv2, ipndm, ipndm_v, lcm}
                                     sampling method (default: "euler_a")
- [] --steps  STEPS                     number of sample steps (default: 20)
- [] --rng {std_default, cuda}          RNG (default: cuda)
- [] -s SEED, --seed SEED               RNG seed (default: 42, use random seed for < 0)
- [] -b, --batch-count COUNT            number of images to generate
- [] --schedule {discrete, karras, exponential, ays, gits} Denoiser sigma schedule (default: discrete)
- [] --clip-skip N                      ignore last layers of CLIP network; 1 ignores none, 2 ignores one layer (default: -1)
                                     <= 0 represents unspecified, will be 1 for SD1.x, 2 for SD2.x
- [] --vae-tiling                       process vae in tiles to reduce memory usage
- [] --vae-on-cpu                       keep vae in cpu (for low vram)
- [] --clip-on-cpu                      keep clip in cpu (for low vram)
- [] --diffusion-fa                     use flash attention in the diffusion model (for low vram)
                                     Might lower quality, since it implies converting k and v to f16.
                                     This might crash if it is not supported by the backend.
- [] --control-net-cpu                  keep controlnet in cpu (for low vram)
- [] --canny                            apply canny preprocessor (edge detection)
- [] --color                            Colors the logging tags according to level
- [xx] -v, --verbose                      print extra info
