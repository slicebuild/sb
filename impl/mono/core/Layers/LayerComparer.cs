using System;
using System.Collections.Generic;

namespace sb.Core.Layers
{
    public class LayerComparer : IComparer<Layer>
    {
        /// <summary>
        /// Layers are sorted by name (asc), then by folder version (desc), then by file version (desc)
        /// </summary>
        /// <param name="x">Layer 1</param>
        /// <param name="y">Layer 2</param>
        /// <returns></returns>
        public int Compare(Layer x, Layer y)
        {
            if (x.SemVerName.Name != y.SemVerName.Name)
                return string.Compare(x.SemVerName.Name, y.SemVerName.Name, StringComparison.Ordinal);
            // compares y to x to get descending ordering
            return y.SemVerName.CompareTo(x.SemVerName);
        }
    }
}