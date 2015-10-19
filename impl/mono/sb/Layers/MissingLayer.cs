using sb.Slices;

namespace sb.Layers
{
    public class MissingLayer : Layer
    {
        public MissingLayer(LayerList registryLayers, string name) 
            : base(registryLayers, new EmptySlice(name))
        {
        }
    }
}