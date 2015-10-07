using System;
using System.Collections.Generic;

namespace cb.Layers
{
    public class LayerComparer : IComparer<Layer>
    {
        //todo: implement priority
        public int Compare(Layer x, Layer y)
        {
            if (x.Name != y.Name)
                return string.Compare(x.Name, y.Name, StringComparison.Ordinal);
            // compares y to x to get descending ordering
            return y.SemVersion.CompareTo(x.SemVersion);
        }
    }
}