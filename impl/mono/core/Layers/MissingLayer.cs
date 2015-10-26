using sb.Core.Slices;

namespace sb.Core.Layers
{
    /// <summary>
    /// MissingLayer is used when the required slice is missing.
    /// </summary>
    public class MissingLayer : Layer
    {
        public MissingLayer(SliceList sliceList, MissingSlice slice) 
            : base(sliceList, slice)
        {
        }
    }
}