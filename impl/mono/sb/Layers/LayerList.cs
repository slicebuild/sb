using System.Collections.Generic;
using sb.Slices;

namespace sb.Layers
{
    /// <summary>
    /// List of all Layers created out of the slices in the slices directory.
    /// </summary>
    public class LayerList : List<Layer>
    {
        /// <summary>
        /// All slices from the slices directory and subdirectories stored in a flat list
        /// become layers in a flat sorted list where they can be easily found
        /// by name or by name + version. Also, each layer looks up it's dependencies.
        /// </summary>
        /// <param name="slices"></param>
        /// <param name="osName"></param>
        public LayerList(IList<Slice> slices, string osName)
        {
            OsName = osName;
            foreach (var slice in slices)
            {
                Add(new Layer(this, slice));
            }
            Sort(new LayerComparer());
            ForEach(l => l.FindDependenciesRecursive(this));
        }

        public string OsName { get; }
        public IList<string> MissingNames { get; } = new List<string>();

        /// <summary>
        /// Layers are sorted by name (asc), then by folder version (desc), then by file version (desc)
        /// FindLayers returns first matching layer with the same name and greatest version or NULL.
        /// If the layer hasn't been found it's name is added to the MissingNames list.
        /// </summary>
        /// <param name="name"></param>
        /// <returns></returns>
        public Layer FindLayer(string name)
        {
            var layer = Find(item => item.SemVerName.Name == name);
            if (layer == null)
            {
                AddMissingName(name);
                layer = new MissingLayer(this, name);
            }
            return layer;
        }

        public IList<Layer> FindLayers(string[] names)
        {
            var layers = new List<Layer>();
            foreach (var name in names)
            {
                var layer = FindLayer(name);
                if (!layers.Contains(layer))
                {
                    layers.Add(layer);
                }
            }
            return layers;
        }

        private void AddMissingName(string name)
        {
            if (!MissingNames.Contains(name))
                MissingNames.Add(name);
        }
    }
}