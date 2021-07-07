const float fsaa = 144.;

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 col = vec3(0.);
    float bound = sqrt(fsaa)-1.;
   	for(float i = -.5*bound; i<=.5*bound; i+=1.)
        for(float j=-.5*bound; j<=.5*bound; j+=1.)
        {
     		col += texture(iChannel0, fragCoord/iResolution.xy+vec2(i,j)*mix(3.,20.,2.*abs(fragCoord.y/iResolution.y-.5))*exp(-abs(1.e-2*length(fragCoord.xy)/iResolution.y-.5))/max(bound, 1.)/iResolution.xy).xyz;
        }
    col /= fsaa;
    fragColor = vec4(col,1.0);
}
