using System;
using System.Collections.Generic;

namespace sb.Layers
{
    public class LayerComparer : IComparer<Layer>
    {
        //todo: implement priority
        public int Compare(Layer x, Layer y)
        {
            if (x.SemVerName.Name != y.SemVerName.Name)
                return string.Compare(x.SemVerName.Name, y.SemVerName.Name, StringComparison.Ordinal);
            // compares y to x to get descending ordering
            return (y.SemVerName.NameVersion as IComparable).CompareTo(x.SemVerName.NameVersion);
        }
    }
}