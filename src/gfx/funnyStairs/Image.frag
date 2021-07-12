const float fsaa = 144.;
const vec3 c = vec3(1.,0.,-1.);
float scale,
    alternateScale,
    nbeats,
    bpm = .5*149.,
    spb =  60. / bpm;
const float tmax = 90.;

float m(vec2 x)
{
    return max(x.x,x.y);
}

float d210(vec2 x)
{
    return min(max(max(max(max(min(max(max(m(abs(vec2(abs(abs(x.x)-.25)-.25, x.y))-vec2(.2)), -m(abs(vec2(x.x+.5, abs(abs(x.y)-.05)-.05))-vec2(.12,.02))), -m(abs(vec2(abs(x.x+.5)-.1, x.y-.05*sign(x.x+.5)))-vec2(.02,.07))), m(abs(vec2(x.x+.5,x.y+.1))-vec2(.08,.04))), -m(abs(vec2(x.x, x.y-.04))-vec2(.02, .08))), -m(abs(vec2(x.x, x.y+.1))-vec2(.02))), -m(abs(vec2(x.x-.5, x.y))-vec2(.08,.12))), -m(abs(vec2(x.x-.5, x.y-.05))-vec2(.12, .07))), m(abs(vec2(x.x-.5, x.y))-vec2(.02, .08)));
}

float sm(in float d)
{
    return smoothstep(1.5/iResolution.y, -1.5/iResolution.y, d);
}

// Creative Commons Attribution-ShareAlike 4.0 International Public License
// Created by David Hoskins.
// See https://www.shadertoy.com/view/4djSRW
float hash12(vec2 p)
{
	vec3 p3  = fract(vec3(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float lfnoise(vec2 t)
{
    vec2 i = floor(t);
    t = fract(t);
    t = smoothstep(c.yy, c.xx, t);
    vec2 v1 = vec2(hash12(i), hash12(i+c.xy)), 
        v2 = vec2(hash12(i+c.yx), hash12(i+c.xx));
    v1 = c.zz+2.*mix(v1, v2, t.y);
    return mix(v1.x, v1.y, t.x);
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Sync tools
    float stepTime = mod(iTime+.5*spb, spb)-.5*spb;
    nbeats = (iTime-stepTime+.5*spb)/spb + smoothstep(-.2*spb, .2*spb, stepTime);
    scale = smoothstep(-.3*spb, 0., stepTime)*smoothstep(.3*spb, 0., stepTime);
    alternateScale = mix(smoothstep(-.4*spb, -.5*spb, stepTime),smoothstep(-.5*spb, -.4*spb, stepTime), mod(round((iTime-stepTime)/spb), 2.));

    // SSAA
    vec3 col = vec3(0.);
    float bound = sqrt(fsaa)-1.;
   	for(float i = -.5*bound; i<=.5*bound; i+=1.)
        for(float j=-.5*bound; j<=.5*bound; j+=1.)
        {
     		col += texture(iChannel0, fragCoord/iResolution.xy+vec2(i,j)*mix(3.,20.,2.*abs(fragCoord.y/iResolution.y-.5))*exp(-abs(1.e-2*length(fragCoord.xy)/iResolution.y-.5))/max(bound, 1.)/iResolution.xy).xyz;
        }
    col /= fsaa;

    vec2 uv = (fragCoord.xy-.5*iResolution.xy)/iResolution.y;

    // edge glow
    vec2 uv2 = uv;
    uv = fragCoord/iResolution.xy;
    vec2 unit = 1./iResolution.xy;
    
    float o = 1.0;
    float p = 3.0;
    float q = 0.0;
    
    
    vec4 col11 = texture(iChannel0, uv + vec2(-unit.x, -unit.y));
    vec4 col12 = texture(iChannel0, uv + vec2( 0., -unit.y));
    vec4 col13 = texture(iChannel0, uv + vec2( unit.x, -unit.y));
    
    vec4 col21 = texture(iChannel0, uv + vec2(-unit.x, 0.));
    vec4 col22 = texture(iChannel0, uv + vec2( 0., 0.));
    vec4 col23 = texture(iChannel0, uv + vec2( unit.x, 0.));
    
    vec4 col31 = texture(iChannel0, uv + vec2(-unit.x, unit.y));
    vec4 col32 = texture(iChannel0, uv + vec2( 0., unit.y));
    vec4 col33 = texture(iChannel0, uv + vec2( unit.x, unit.y));
    
    vec4 x = col11 * -o + col12 * -p + col13 * -o + col31 * o + col32 * p + col33 * o + col22 * q;
    vec4 y = col11 * -o + col21 * -p + col31 * -o + col13 * o + col23 * p + col33 * o + col22 * q;
    
    // Output to screen
    fragColor = vec4(abs(y.rgb) * 0.5 + abs(x.rgb) * 0.5, 1.);
    fragColor = vec4(mix(col, fragColor.rgb, clamp(/*(.25+.5*lfnoise(.5*nbeats*c.xx))+*/.6*alternateScale,0.,1.)),1.0);

    // TODO: Sparkle

    // Vignette
    uv = fragCoord.xy / iResolution.xy;
    uv *=  1.0 - uv.yx;
    float vig = uv.x*uv.y * 15.0;
    vig = pow(vig, 0.2);
    fragColor *= vig; 

    // team210 watermark
    float d = d210(8.*(uv2-.5*vec2(iResolution.x/iResolution.y,1.)+vec2(.1,.04)));
    fragColor.rgb = mix(fragColor.rgb, mix(fragColor.rgb, c.xxx, .5), sm(d));
}
