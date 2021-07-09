const vec3 c = vec3(1.,0.,-1.);
const float pi = 3.14159,
    PHI = 1.618,
    bpm = .5*149.,
    spb =  60. / bpm;
mat3 RR = mat3(1.),
    RRA = mat3(1.);
float scale,
    nbeats;
const float tmax = 80.521;

const int NM = 41;
const float syncMagics[NM] = float[NM](
    // Intro synth part
    0,1,2,3,6,7,
    8,9,11,12,13,14,
    16,17,18,19,22,23,
    24,25,27,29,
    32,33,34,35,38,39,
    40,41,43,44,45,46,47,
    48,49,50,51,53,55
);

// iq's code
float smoothmin(float a, float b, float k)
{
    float h = max( k-abs(a-b), 0.0 )/k;
    return min( a, b ) - h*h*h*k*(1.0/6.0);
}

float smoothmax(float a, float b, float k)
{
    return a + b - smoothmin(a,b,k);
}

float zextrude(float z, float d2d, float h)
{
    vec2 w = vec2(d2d, abs(z)-0.5*h);
    return min(max(w.x,w.y),0.0) + length(max(w,0.0));
}

void dhexagonpattern(in vec2 p, out float d, out vec2 ind) 
{
    vec2 q = vec2( p.x*1.2, p.y + p.x*0.6 );
    
    vec2 pi = floor(q);
    vec2 pf = fract(q);

    float v = mod(pi.x + pi.y, 3.0);

    float ca = step(1.,v);
    float cb = step(2.,v);
    vec2  ma = step(pf.xy,pf.yx);
    
    d = dot( ma, 1.0-pf.yx + ca*(pf.x+pf.y-1.0) + cb*(pf.yx-2.0*pf.xy) );
    ind = pi + ca - cb*ma;
    ind = vec2(ind.x/1.2, ind.y);
    ind = vec2(ind.x, ind.y-ind.x*.6);
}

mat3 rot3(in vec3 p)
{
    return mat3(c.xyyy, cos(p.x), sin(p.x), 0., -sin(p.x), cos(p.x))
        *mat3(cos(p.y), 0., -sin(p.y), c.yxy, sin(p.y), 0., cos(p.y))
        *mat3(cos(p.z), -sin(p.z), 0., sin(p.z), cos(p.z), c.yyyx);
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

// Creative Commons Attribution-ShareAlike 4.0 International Public License
// Created by David Hoskins.
// See https://www.shadertoy.com/view/4djSRW
vec2 hash22(vec2 p)
{
	vec3 p3 = fract(vec3(p.xyx) * vec3(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx+33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
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

float mfnoise(vec2 x, float d, float b, float e)
{
    float n = 0.;
    float a = 1., nf = 0., buf;
    for(float f = d; f<b; f *= 2.)
    {
        n += a*lfnoise(f*x-2.*iTime);
        a *= e;
        nf += 1.;
    }
    return n * (1.-e)/(1.-pow(e, nf));
}

vec3 hsv2rgb(vec3 cc)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(cc.xxx + K.xyz) * 6.0 - K.www);
    return cc.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), cc.y);
}

vec3 rgb2hsv(vec3 cc)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(cc.bg, K.wz), vec4(cc.gb, K.xy), step(cc.b, cc.g));
    vec4 q = mix(vec4(p.xyw, cc.r), vec4(cc.r, p.yzx), step(p.x, cc.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

float dbox3(vec3 x, vec3 b)
{
    vec3 da = abs(x) - b;
    return length(max(da,0.0))
            + min(max(da.x,max(da.y,da.z)),0.0);
}

// Distance to spiral
float spiral(in vec2 x, in float k)
{
    float tau = 2.*pi;
    vec2 dpr = mod(vec2(atan(x.y,x.x),length(x)/k),tau);
    float a = abs(dpr.y-dpr.x);
    return k*min(a,tau-a);
}

// Distance to line segment
float linesegment(in vec2 x, in vec2 p1, in vec2 p2)
{
    vec2 da = p2-p1;
    return length(x-mix(p1, p2, clamp(dot(x-p1, da)/dot(da,da),0.,1.)));
}

// Distance to star
float star(in vec2 x, in float r1, in float r2, in float N)
{
    N *= 2.;
    float p = atan(x.y,x.x),
        k = pi/N,
    	dp = mod(p+pi, 2.*k),
    	parity = mod(round((p+pi-dp)*.5/k), 2.),
        dk = k,
        dkp = mix(dk,-dk,parity);
    
    vec2 p1 = r1*vec2(cos(k-dkp),sin(k-dkp)),
        p2 = r2*vec2(cos(k+dkp),sin(k+dkp)),
        dpp = p2-p1,
        n = normalize(p2-p1).yx*c.xz, 
        xp = length(x)*vec2(cos(dp), sin(dp));
    float t = dot(xp-p1,dpp)/dot(dpp,dpp);
    float r = mix(1.,-1.,parity)*dot(xp-p1,n);
    if(t < 0.)
        return sign(r)*length(xp-p1);
    else if(t > 1.)
        return sign(r)*length(xp-p2);
    else
	    return r;
}

float m(vec2 x)
{
    return max(x.x,x.y);
}

float d210(vec2 x)
{
    return min(max(max(max(max(min(max(max(m(abs(vec2(abs(abs(x.x)-.25)-.25, x.y))-vec2(.2)), -m(abs(vec2(x.x+.5, abs(abs(x.y)-.05)-.05))-vec2(.12,.02))), -m(abs(vec2(abs(x.x+.5)-.1, x.y-.05*sign(x.x+.5)))-vec2(.02,.07))), m(abs(vec2(x.x+.5,x.y+.1))-vec2(.08,.04))), -m(abs(vec2(x.x, x.y-.04))-vec2(.02, .08))), -m(abs(vec2(x.x, x.y+.1))-vec2(.02))), -m(abs(vec2(x.x-.5, x.y))-vec2(.08,.12))), -m(abs(vec2(x.x-.5, x.y-.05))-vec2(.12, .07))), m(abs(vec2(x.x-.5, x.y))-vec2(.02, .08)));
}

// Scene marching information
struct SceneData
{
    float

        // Material for palette
        material,
    
        // Distance
        dist,
    
        // Light accumulation for clouds
        accumulation,
    
        // Reflectivity
        reflectivity,
    
        // Transmittivity
        transmittivity,
    
        // Illumination
        specular,
    
        // Diffuse
        diffuse;
};

SceneData defaultMaterial(float d)
{
    return SceneData(1.3, d, 1., .1, .1, .5, 1.);
}

SceneData add(SceneData a, SceneData b)
{
    if(a.dist < b.dist) return a;
    return b;
}

float rj;

float effect1(vec3 x, float zj, float r, float s)
{
    // star effect
    float ag = mix(2.,12.,.5+.5*r)*zj*r;
    mat2 RB = mat2(cos(ag), sin(ag), -sin(ag), cos(ag));
    float da = -abs(star(RB*(x.xy-vec2(r,s)*.5), abs(1.*r+.1*zj), abs(1.*s-.1*zj), round(5.+r+s)))+.01-.1*zj,
        db = mod(da, .2)-.09*2.1;
    rj = da - db;
    return db;
}

float effect2(vec3 x, float zj, float r, float s)
{
    // noise
    return -1.+mfnoise(x.xy-r*.3, 3., 1.e1, .45)-3.*zj;
}

float effect3(vec3 x, float zj, float r, float s)
{
    // spiral effect
    mat2 RA = mat2(cos(iTime), sin(iTime), -sin(iTime), cos(iTime));
    return -abs(spiral(RA*RA*(x.xy)-.3*r, mix(.05,.1,.5+.5*r)))-.3*zj+.01*r;
}

float effect4(vec3 x, float zj, float r, float s)
{
    // Team210 logo
    float rsize = .3;
    float da = -abs(mod(d210(x.xy-zj*.4),rsize)+.5*rsize-.4-.2*r-.5*zj)+.01+.01*scale+.001*zj;
    return da;
    // return -abs(da) + .01 - .5*zj;
    // circle tornado
    // float rsize = .3;
    // return -abs(mod(length(x.xy-zj*.4),rsize)+.5*rsize-.4-.2*r-.5*zj)+.01+.01*scale+.001*zj;
}

float effect5(vec3 x, float zj, float r, float s)
{
    // hexagon style
    vec2 vi;
    float vsize = 2.+2.*r,
        v;
    dhexagonpattern(vsize*x.xy, v, vi);
    return -abs(v / vsize) + .01 - .5*zj;
}

float effect6(vec3 x, float zj, float r, float s)
{
    // Steckenmist, sieht fet aus denk ich
    const float aside = .4,
        psize = pi/6.,
        msize = .5;
    vec2 rp = vec2(atan(x.y,x.x), length(x.xy));
    float dp = mod(rp.x, psize)-.5*psize,
        pj = rp.x-dp,
        dr = mod(rp.y, msize)-.5*msize,
        rj = rp.y-dr;
    
    vec2 yj = (rj - .2*sin(pi*zj-r)) * vec2(cos(pj), sin(pj)),
        aj = rp.y * vec2(cos(rp.x), sin(rp.x));
    float da = -length(mat2(cos(iTime-zj), sin(iTime-zj), -sin(iTime-zj), cos(iTime-zj))*(x.xy-yj)) +.001 +.1*(.5+.5*s)+.05*(.6+.4*scale)+.01*zj*(.5+.5*r);
    return mod(da, .2)-.09*2.1;
}

float holeSDF(vec3 x, float zj)
{
    float r = lfnoise(.5*nbeats*c.xx-zj),
        s = lfnoise(.5*nbeats*c.xx+1337.-zj);

    float selector = 1.-clamp(iTime/tmax,0.,1.);
    //lfnoise(.05*nbeats*c.xx+133.);
    // selector = .5+.5*selector;
    const float N = 6.;

    if(selector < 1.5/N)
    {
        return mix(effect1(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(.1/N, 0., selector)*smoothstep(1.4/N, 1.5/N, selector));
        // return mix(effect1(x, zj, r, s), effect2(x, zj, r, s), smoothstep(1.4/N, 1.5/N, selector));
    }
    else if(selector < 3./N)
    {
        return mix(effect2(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(1.6/N, 1.5/N, selector)*smoothstep(2.9/N, 3./N, selector));
        // return mix(effect2(x, zj, r, s), effect3(x, zj, r, s), smoothstep(2.9/N, 3./N, selector));
    }
    else if(selector < 3.5/N)
    {
        return mix(effect3(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(3.1/N, 3./N, selector)*smoothstep(3.4/N, 3.5/N, selector));
        // return mix(effect3(x, zj, r, s), effect4(x, zj, r, s), smoothstep(3.4/N, 3.5/N, selector));
    }
    else if(selector < 4./N)
    {
        return mix(effect4(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(3.6/N, 3.5/N, selector)*smoothstep(3.9/N, 4./N, selector));
        // return mix(effect4(x, zj, r, s), effect5(x, zj, r, s), smoothstep(3.9/N, 4./N, selector));
    }
    else if(selector < 5./N)
    {
        return mix(effect5(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(4.1/N, 4./N, selector)*smoothstep(4.9/N, 5./N, selector));
        // return mix(effect5(x, zj, r, s), effect6(x, zj, r, s), smoothstep(4.9/N, 5./N, selector));
    }
    else
    {
        return mix(effect6(x, zj, r, s), -abs(length(x.xy)-.3+.05*zj) + .01 - .5*zj, smoothstep(5.1/N, 5./N, selector)*smoothstep(5.9/N, 6./N, selector));
        // return effect6(x, zj, r, s);
    }
}

SceneData scene(vec3 x)
{
    SceneData sdf = SceneData(0., x.z+.5, 0., 0., 0., .7, 1.);

    float dz = .03,
        z = mod(x.z, dz) - .5 * dz,
        zj = x.z - z,
        zjz = zj / dz;

    if(zj <= 0.)
    {
        vec3 d2d = -vec3(holeSDF(x,zj-dz), holeSDF(x, zj), holeSDF(x, zj+dz));
        float d = smoothmin(
            smoothmin(
                zextrude(z-dz, d2d.x, .5*dz)-.15*dz,
                zextrude(z, d2d.y, .5*dz)-.15*dz,
                .01
            ),
            zextrude(z+dz, d2d.z, .5*dz)-.15*dz,
            .01
        );
        sdf = add(
            sdf,
            SceneData(-1.+3.*abs(zjz/.5*dz), d, 0., 0., 0., .7, 1.)
        );
    }

    return sdf;
}

vec3 normal(vec3 x)
{
    float s = scene(x).dist,
        dx = 5.e-5;
    return normalize(vec3(
        scene(x+dx*c.xyy).dist, 
        scene(x+dx*c.yxy).dist, 
        scene(x+dx*c.yyx).dist
    )-s);
}

vec3 palette(float scale)
{
    const int N = 4;
    vec3 colors[N] = vec3[N](
        // .8*c.xxx,
        vec3(1.00,0.22,0.30),
        c.yyy,
        vec3(0.13,0.44,0.66),
        vec3(0.00,0.80,0.73)
    );
    float i = floor(scale),
        ip1 = mod(i + 1., float(N));
    return mix(colors[int(i)], colors[int(ip1)], fract(scale));
}

bool ray(out vec3 col, out vec3 x, inout float d, vec3 dir, out SceneData s, vec3 o, vec3 l, out vec3 n)
{
    for(int i=0-min(iFrame, 0); i<250+min(iFrame,0); ++i)
    {
        x = o + d * dir;
        s = scene(x);
        
        if(s.dist < 1.e-4) 
        {
            // Blinn-Phong Illumination
            n = normal(x);

            if(s.material == 0.)
            {
                col = c.yyy;
            }
            else 
            {
                col = palette(s.material+rj*10. - .1*length(x.xy));
            }

            col = .2 * col
                + s.diffuse * col*max(dot(normalize(l-x),n),0.)
                + s.specular * col*pow(max(dot(reflect(normalize(l-x),n),dir),0.),2.);

            return true;
        }
        
        d += min(s.dist,s.dist>1.e0?1.e-2:5.e-3);
        // d += s.dist;
    }
    return false;
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Rotation tools
    RR = rot3(iTime*vec3(0.,0.,.6));
    RRA = rot3(iTime*vec3(.7,.9,1.32));

    // Sync tools
    float stepTime = mod(iTime, spb)-.5*spb;
    nbeats = (iTime-stepTime-.5)/spb + smoothstep(-.2*spb, .2*spb, stepTime);
    scale = smoothstep(-.3*spb, 0., stepTime)*smoothstep(.3*spb, 0., stepTime);

    // Marching tools
    float d = 0.,
        d1;
    vec2 uv = (fragCoord.xy-.5*iResolution.xy)/iResolution.y;
    vec3 o = RR*c.yzx,
        col = c.yyy,
        c1 = c.yyy,
        x,
        x1,
        n,
        n1,
        r = RR*c.xyy,
        t = c.yyy,
        dir = normalize(uv.x * r + uv.y * cross(r,normalize(t-o))-o),
        l = c.zzx;
    SceneData s, 
        s1;

    d = -(o.z)/dir.z;
    x = o + d * dir;
        
    // Material ray
    if(ray(col, x, d, dir, s, o, l, n))
    {
        // Reflections
        if(s.reflectivity > 0.)
        {
            d1 = 2.e-3;
            if(ray(c1, x1, d1, reflect(dir,n), s1, x, l, n1))
                col = mix(col, c1, s.reflectivity);
        }

        // Refractions
        if(s.transmittivity > 0.)
        {
            d1 = 2.e-3;
            if(ray(c1, x1, d1, refract(dir,n, .99), s1, x, l, n1))
                col = mix(col, c1, s.transmittivity);
        }

        s1 = s;
        d1 = d;
        n1 = n;

        
        // Ambient occlusion
        // float calcOcclusion( in vec3 pos, in vec3 nor, float ra )
        float occ = 0.;
        for(int i=0; i<32; ++i)
        {
            float h = .01 + 4.0*pow(float(i)/31.0,2.0);
            vec2 an = hash22( hash12(iTime*c.xx)*c.xx + float(i)*13.1 )*vec2( 3.14159, 6.2831 );
            vec3 dir2 = vec3( sin(an.x)*sin(an.y), sin(an.x)*cos(an.y), cos(an.x) );
            dir2 *= sign( dot(dir2,n) );
            occ += clamp( 5.0*scene( x + h*dir2 ).dist/h, -1.0, 1.0);
        }
        col = mix(sqrt(col), col, clamp(occ/32.,0.,1.));

        // Soft shadow
        if(x.z <= .1)
        {
            // Soft Shadow
            o = x;
            dir = normalize(l-x);
            d1 = 1.e-2;
            
            // if(d < 1.e2)
            {
                float res = 1.0;
                float ph = 1.e20;
                for(int i=0; i<250; ++i)
                // for(d=1.e-2; x.z<.5; )
                {
                    x = o + d1 * dir;
                    s = scene(x);
                    if(s.dist < 1.e-4) 
                    {
                        res = 0.;
                        break;
                    }
                    if(x.z >= .1) // 0? 
                    {
                        res = 1.;
                        break;
                    }
                    float y = s.dist*s.dist/(2.0*ph)/12.;
                    float da = sqrt(s.dist*s.dist-y*y);
                    res = min( res, 100.0*da/max(0.0,d1-y) );
                    ph = s.dist;
                    d1 += min(s.dist,s.dist>5.e-1?1.e-2:5.e-3);
    //                d1 += min(s.dist,s.dist>1.e-1?1.e-2:5.e-3);
                }
                col = mix(.5*col, col, res);
            }
        }
    }

    s = s1;

    // Color drift
    if(s.material != 0.)
    {
        c1 = rgb2hsv(col);
        c1.r = pi*lfnoise(.1*nbeats*c.xx);
        col = mix(col, hsv2rgb(c1),.5);
        
        // Gamma
        col = col + col*col + col*col*col;
        // col *= col; 
    }

    // Highlights
    col = mix(col, mix(col, col + col*col + col*col*col,.5), smoothstep(.9, 1.4, abs(dot(c.xzx, n))));
    
    // fog (looks crap)
    // col = mix(col, palette(length(uv)), smoothstep(.1,.5, d1));

    // Fade from and to black
    col = mix(c.yyy, col, smoothstep(0.,1.,iTime)*smoothstep(tmax+10,tmax,iTime));

    fragColor = mix(texture(iChannel0, fragCoord.xy/iResolution.xy), vec4(clamp(col,0.,1.),1.), .5);
}
