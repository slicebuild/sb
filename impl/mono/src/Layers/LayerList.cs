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
        /// Layers are sorted by name (asc), then by folder version (desc), then by file version (desc)
        /// FindLayer returns first matching layer with the same name and greatest version(s)
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
                foreach (var layer in this.Where(l => l.SemVerName.Name.ToLowerInvariant().Contains(name)))
                {
                    if (!layers.Contains(layer))
                        layers.Add(layer);
                }
            }
            return layers;
        }
    }
}