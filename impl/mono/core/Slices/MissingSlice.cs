using System;
using sb.Core.Utils;

namespace sb.Core.Slices
{
    public class MissingSlice : Slice
    {
        public MissingSlice(SemVerInfo svi) 
            : base(svi.Label, svi, new string[0])
        {
        }
    }
}