using System.Collections.Generic;
using System.Linq;
using sb.Slices;

namespace sb.Layers
{
    public class LayerList : List<Layer>
    {
        public LayerList()
        {
        }

        public LayerList(IList<Slice> slices)
        {
            foreach (var slice in slices)
            {
                Add(new Layer(slice));
            }
            Sort(new LayerComparer());
            ForEach(l => l.FindDependenciesRecursive(this));
        }

        /// <summary>
        /// Layers are sorted by name (asc), version (desc) and priority (desc).
        /// FindLayer returns first matching layer with the same name and greatest version
        /// If name and version match then priority is applied - the layer with higher priority is returned
        /// </summary>
        /// <param name="name"></param>
        /// <returns></returns>
        public Layer FindLayer(string name)
        {
            return Find(l => l.SemVerName.Name == name);
        }

        public LayerList FindLayers(string[] names)
        {
            var layers = new LayerList();
            foreach (var name in names)
            {
                layers.AddRange(this.Where(l => l.SemVerName.Name.ToLowerInvariant().Contains(name)));
            }
            return layers;
        }
    }
}