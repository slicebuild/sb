using System;
using System.Collections.Generic;
using System.Linq;
using sb.Core.Layers;
using sb.Core.Slices;
using sb.Core.Utils;

namespace sb.Core.App.Commands
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
            var layers = BuildLayers();

            ReportAllSimilar(layers);
            ReportAllMissing(layers);
            ReportFoundRequested(layers);
            ReportMissingRequested(layers);
        }

        /// <summary>
        /// Finds all slices requested on the cmd line and 
        /// build the layers into a list 
        /// </summary>
        /// <returns></returns>
        protected virtual LayerList BuildLayers()
        {
            var layerParams = Args.GetLayerParams();
            var osParam = Args.GetOsParam();

            var layerInfos = layerParams.Select(layerParam => new SemVerInfo(layerParam)).ToList();
            var osInfo = new SemVerInfo(osParam);

            var dirList = new SliceDirectoryList(Args.SlicesDir, Args.VersionInfo.FileMajorPart);
            var sliceList = dirList.Scan(osInfo);

            var layerList = new LayerList(sliceList, osInfo, layerInfos);
            return layerList;
        }

        /// <summary>
        /// Find all layers with names similar to the requested
        /// </summary>
        /// <param name="layerList"></param>
        protected virtual void ReportAllSimilar(LayerList layerList)
        {
            var similar = layerList.SliceList.FindSimilar(layerList.LayerInfos);

            Console.WriteLine();
            if (similar.Count == 0)
            {
                Console.WriteLine("All Similar: None");
            }
            else
            {
                Console.WriteLine("All Similar:");
                foreach (var slice in similar)
                {
                    Console.WriteLine(slice.RelPath);
                }
            }
        }

        /// <summary>
        /// All missing layers in the registry
        /// </summary>
        /// <param name="layerList"></param>
        protected virtual void ReportAllMissing(LayerList layerList)
        {
            Console.WriteLine();
            if (layerList.SliceList.MissingInfos.Count == 0)
            {
                Console.WriteLine("All Missing: None");
            }
            else
            {
                Console.WriteLine("All Missing:");
                foreach (var info in layerList.SliceList.MissingInfos)
                {
                    Console.WriteLine(info.Name);
                }
            }
        }

        /// <summary>
        /// Found requested layers
        /// </summary>
        /// <param name="layerList"></param>
        protected virtual void ReportFoundRequested(LayerList layerList)
        {
            var foundLayers = new List<Layer>();
            foreach (var info in layerList.LayerInfos)
            {
                var layer = layerList.FindLayer(info);
                if (!(layer is MissingLayer))
                    foundLayers.Add(layer);
            }
            Console.WriteLine();
            if (foundLayers.Count == 0)
            {
                Console.WriteLine("Found Requested: None");
            }
            else
            {
                Console.WriteLine("Found Requested:");
                foreach (var layer in foundLayers)
                {
                    Console.WriteLine(layer.Slice.RelPath);
                }
            }
        }

        /// <summary>
        /// Missing dependencies of the requested layers
        /// </summary>
        /// <param name="layerList"></param>
        protected virtual void ReportMissingRequested(LayerList layerList)
        {
            Console.WriteLine();
            if (layerList.MissingInfos.Count == 0)
            {
                Console.WriteLine("Missing Requested: None");
            }
            else
            {
                Console.WriteLine("Missing Requested:");
                Console.WriteLine(string.Join(Environment.NewLine, layerList.MissingInfos.Select(item => item.Name)));
            }
        }
    }
}