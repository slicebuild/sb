using System;
using System.Collections.Generic;

namespace sb.Core.Layers
{
    public class LayerComparer : IComparer<Layer>
    {
        /// <summary>
        /// Layers are sorted by the bunch version (desc), then by file version (desc), then by name
        /// </summary>
        /// <param name="x">Layer 1</param>
        /// <param name="y">Layer 2</param>
        /// <returns></returns>
        public int Compare(Layer x, Layer y)
        {
            return y.SemVerName.CompareTo(x.SemVerName);
        }
    }
}