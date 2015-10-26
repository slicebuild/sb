using System.Collections;
using System.Collections.Generic;
using sb.Core.Slices;
using sb.Core.Utils;

namespace sb.Core.Layers
{
    public class LayerList : IEnumerable<Layer>
    {
        private readonly List<Layer> _layers = new List<Layer>();

        public LayerList(SliceList sliceList, SemVerInfo osInfo, IList<SemVerInfo> layerInfos)
        {
            SliceList = sliceList;
            OsInfo = osInfo;
            LayerInfos = layerInfos;

            // first put the requested layers into the list
            foreach (var layerInfo in layerInfos)
            {
                FindLayer(layerInfo);
            }

            // then add the dependencies
            foreach (var layerInfo in layerInfos)
            {
                FindLayer(layerInfo).FindDependenciesRecursive(this);
            }

            // then if there were more than one layers requested
            // add other layers as the dependencies for the first one
            for (var i = layerInfos.Count - 1; i > 0; i--)
            {
                _layers[0].InsertDependency(0, FindLayer(layerInfos[i]));
            }
        }

        public SliceList SliceList { get; }
        public SemVerInfo OsInfo { get; }
        public IList<SemVerInfo> LayerInfos { get; }
        public IList<SemVerInfo> MissingInfos { get; } = new List<SemVerInfo>();

        public Layer this[int index] => _layers[index];

        public void Add(Layer layer)
        {
            if (!_layers.Contains(layer))
                _layers.Add(layer);
        }

        public Layer FindLayer(SemVerInfo layerInfo)
        {
            var layer =
                _layers.Find(
                    item => item.Slice.SemVerInfo.Name == layerInfo.Name && item.Slice.SemVerInfo.CompareByNameSemVer(layerInfo) >= 0);

            if (layer == null)
            {
                var slice = SliceList.FindSlice(layerInfo);
                if (slice is MissingSlice)
                {
                    if (!MissingInfos.Contains(layerInfo))
                        MissingInfos.Add(layerInfo);
                    layer = new MissingLayer(SliceList, (MissingSlice) slice);
                }
                else
                {
                    layer = new Layer(SliceList, slice);
                }
                Add(layer);
            }

            return layer;
        }

        public IEnumerator<Layer> GetEnumerator()
        {
            return _layers.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return ((IEnumerable) _layers).GetEnumerator();
        }
    }
}