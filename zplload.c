/*

int pack_getc(PACKFILE *f);
Returns the next character from the stream `f', or EOF if the end of the file has been reached.

long pack_igetl(PACKFILE *f);
Like pack_getc, but reads a 32-bit long from a file, using Intel byte ordering (least significant byte first, a.k.a. little-endian).

long pack_igetl(PACKFILE *f);
Like pack_getc, but reads a 32-bit long from a file, using Intel byte ordering (least significant byte first, a.k.a. little-endian).v
 
int pack_igetw(PACKFILE *f);
Like pack_getc, but reads a 16-bit word from a file, using Intel byte ordering (least significant byte first, a.k.a. little-endian).
 */




int readcolordata(PACKFILE *f, miscQdata *Misc, word version, word build, word, word, bool keepdata)
{
    word s_version=0;
    
    miscQdata temp_misc;
    memcpy(&temp_misc, Misc, sizeof(temp_misc));
    
    byte temp_colordata[48];
    char temp_palname[PALNAMESIZE];
    
    int dummy;
    word palcycles;
    
    if(version > 0x192)
    {
        //section version info
        if(!p_igetw(&s_version,f,true))
        {
            return qe_invalid;
        }
        
        //al_trace("Color data version %d\n", s_version);
        if(!p_igetw(&dummy,f,true))
        {
            return qe_invalid;
        }
        
        //section size
        if(!p_igetl(&dummy,f,true))
        {
            return qe_invalid;
        }
    }
    
    //finally...  section data
    for(int i=0; i<oldpdTOTAL; ++i)
    {
        memset(temp_colordata, 0, 48);
        
        if(!pfread(temp_colordata,48,f,true))
        {
            return qe_invalid;
        }
        
        if(keepdata==true)
        {
            memcpy(&colordata[i*48], temp_colordata, 48);
        }
    }
    
    if((version < 0x192)||((version == 0x192)&&(build<73)))
    {
        if(keepdata==true)
        {
            memcpy(colordata+(newerpoSPRITE*48), colordata+(oldpoSPRITE*48), 30*16*3);
            memset(colordata+(oldpoSPRITE*48), 0, ((newerpoSPRITE-oldpoSPRITE)*48));
            memcpy(colordata+((newerpoSPRITE+11)*48), colordata+((newerpoSPRITE+10)*48), 48);
            memcpy(colordata+((newerpoSPRITE+10)*48), colordata+((newerpoSPRITE+9)*48), 48);
            memcpy(colordata+((newerpoSPRITE+9)*48), colordata+((newerpoSPRITE+8)*48), 48);
            memset(colordata+((newerpoSPRITE+8)*48), 0, 48);
        }
    }
    else
    {
        memset(temp_colordata, 0, 48);
        
        for(int i=0; i<newpdTOTAL-oldpdTOTAL; ++i)
        {
            if(!pfread(temp_colordata,48,f,true))
            {
                return qe_invalid;
            }
            
            if(keepdata==true)
            {
                memcpy(&colordata[(oldpdTOTAL+i)*48], temp_colordata, 48);
            }
        }
        
        if(s_version < 4)
        {
            if(keepdata==true)
            {
                memcpy(colordata+(newerpoSPRITE*48), colordata+(newpoSPRITE*48), 30*16*3);
                memset(colordata+(newpoSPRITE*48), 0, ((newerpoSPRITE-newpoSPRITE)*48));
            }
        }
        else
        {
            for(int i=0; i<newerpdTOTAL-newpdTOTAL; ++i)
            {
                if(!pfread(temp_colordata,48,f,true))
                {
                    return qe_invalid;
                }
                
                if(keepdata==true)
                {
                    memcpy(&colordata[(newpdTOTAL+i)*48], temp_colordata, 48);
                }
            }
        }
    }
    
    if((version < 0x192)||((version == 0x192)&&(build<76)))
    {
        if(keepdata==true)
        {
            init_palnames();
        }
    }
    else
    {
        int palnamestoread = 0;
        
        if(s_version < 3)
            palnamestoread = OLDMAXLEVELS;
        else
            palnamestoread = 512;
            
        for(int i=0; i<palnamestoread; ++i)
        {
            memset(temp_palname, 0, PALNAMESIZE);
            
            if(!pfread(temp_palname,PALNAMESIZE,f,true))
            {
                return qe_invalid;
            }
            
            if(keepdata==true)
            {
                memcpy(palnames[i], temp_palname, PALNAMESIZE);
            }
        }
        
        if(keepdata)
        {
            for(int i=palnamestoread; i<MAXLEVELS; i++)
            {
                memset(palnames[i], 0, PALNAMESIZE);
            }
        }
    }
    
    if(version > 0x192)
    {
        for(int i=0; i<256; i++)
        {
            for(int j=0; j<3; j++)
            {
                temp_misc.cycles[i][j].first=0;
                temp_misc.cycles[i][j].count=0;
                temp_misc.cycles[i][j].speed=0;
            }
        }
        
        if(!p_igetw(&palcycles,f,true))
        {
            return qe_invalid;
        }
        
        for(int i=0; i<palcycles; i++)
        {
            for(int j=0; j<3; j++)
            {
                if(!p_getc(&temp_misc.cycles[i][j].first,f,true))
                {
                    return qe_invalid;
                }
            }
            
            for(int j=0; j<3; j++)
            {
                if(!p_getc(&temp_misc.cycles[i][j].count,f,true))
                {
                    return qe_invalid;
                }
            }
            
            for(int j=0; j<3; j++)
            {
                if(!p_getc(&temp_misc.cycles[i][j].speed,f,true))
                {
                    return qe_invalid;
                }
            }
        }
        
        if(keepdata==true)
        {
            memcpy(Misc, &temp_misc, sizeof(temp_misc));
        }
    }
    
    return 0;
}