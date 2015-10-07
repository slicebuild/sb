using System;
using System.Collections.Generic;
using cb.Layers;
using cb.Slices;

namespace cb.App.Commands
{
    public class Find : ICommand
    {
        public Find(Args args)
        {
            Args = args;
        }

        public Args Args { get; }

        /// <summary>
        /// Finds all versions of requested layers for specific OS and lists them.
        /// </summary>
        void ICommand.Run()
        {
            var layerParams = Args.GetLayerParams();
            var osParam = Args.GetOsParam();

            var list = new List<Slice>();

            var dirList = new SliceDirectoryList(Args.SlicesDir);
            dirList.ForEach(dir => list.AddRange(dir.FindByOs(osParam)));

            var layers = new LayerList(list).FindLayers(layerParams);
            foreach (var layer in layers)
            {
                Console.WriteLine(layer.Name);
            }
        }
    }
}