using System;
using System.Collections.Generic;
using sb.Utils;

namespace sb.Slices
{
    public class EmptySlice : Slice
    {
        public EmptySlice(string name) 
            : base(name, 
                  new SemVerName(
                      name, 
                      name,
                      new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0), 
                      new Tuple<int, int, int, int, int, int>(0, 0, 0, 0, 0, 0)), 
                  new string[0])
        {
        }
    }
}