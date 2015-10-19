using sb.Core.Slices;

namespace sb.Core.Layers
{
    /// <summary>
    /// MissingLayer is used when the name can't be found in the registry.
    /// </summary>
    public class MissingLayer : Layer
    {
        public MissingLayer(LayerList registryLayers, string name) 
            : base(registryLayers, new EmptySlice(name))
        {
        }
    }
}