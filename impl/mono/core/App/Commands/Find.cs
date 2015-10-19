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
            var missingNamesList = new List<string>();
            var layers = FindLayers(missingNamesList);

            ReportAllSimilar(layers);
            ReportAllMissing(layers);
            ReportFoundRequested(layers);
            ReportMissingRequested(layers, missingNamesList);
        }

        /// <summary>
        /// Finds all layers requested on the cmd line.
        /// </summary>
        /// <returns></returns>
        protected virtual IList<Layer> FindLayers(IList<string> missingNamesList)
        {
            var layerParams = Args.GetLayerParams();
            var osParam = Args.GetOsParam();

            var list = new List<Slice>();
            var os = SemVerNameParser.Parse(osParam);

            var dirList = new SliceDirectoryList(Args.SlicesDir);
            dirList.ForEach(dir => list.AddRange(dir.FindByOs(os.Name)));

            var registryLayers = new LayerList(list, os.Name);
            var layers = registryLayers.FindLayers(layerParams);

            if (layers.Count > 0)
            {
                // if there were more layers requested, 
                // make them dependencies of the first layer
                for (var i = layers.Count - 1; i >= 1; i--)
                {
                    layers[0].Dependencies.Insert(0, layers[i]);
                }
            }

            WriteMissingNames(layers[0], missingNamesList);
            return layers;
        }

        /// <summary>
        /// Find all layers with names similar to the requested
        /// </summary>
        /// <param name="layers"></param>
        protected virtual void ReportAllSimilar(IList<Layer> layers)
        {
            var similarLayers = new List<Layer>();
            foreach (var layer in layers)
            {
                foreach (var registryLayer in layer.RegistryLayers)
                {
                    if (registryLayer.SemVerName.Name.Contains(layer.SemVerName.Name))
                        similarLayers.Add(registryLayer);
                }
            }
            Console.WriteLine();
            if (similarLayers.Count == 0)
            {
                Console.WriteLine("All Similar: None");
            }
            else
            {
                Console.WriteLine("All Similar:");
                foreach (var layer in similarLayers)
                {
                    Console.WriteLine(layer.RelPath);
                }
            }
        }

        /// <summary>
        /// All missing layers in the registry
        /// </summary>
        /// <param name="layers"></param>
        protected virtual void ReportAllMissing(IList<Layer> layers)
        {
            Console.WriteLine();
            if (layers[0].RegistryLayers.MissingNames.Count == 0)
            {
                Console.WriteLine("All Missing: None");
            }
            else
            {
                Console.WriteLine("All Missing:");
                foreach (var name in layers[0].RegistryLayers.MissingNames)
                {
                    Console.WriteLine(name);
                }
            }
        }

        /// <summary>
        /// Found requested layers
        /// </summary>
        /// <param name="layers"></param>
        protected virtual void ReportFoundRequested(IList<Layer> layers)
        {
            var foundLayers = layers.Where(item => !(item is MissingLayer)).ToList();
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
                    Console.WriteLine(layer.RelPath);
                }
            }
        }

        /// <summary>
        /// Missing dependencies of the requested layers
        /// </summary>
        /// <param name="layers"></param>
        /// <param name="missingNamesList"></param>
        protected virtual void ReportMissingRequested(IList<Layer> layers, IList<string> missingNamesList)
        {
            Console.WriteLine();
            if (missingNamesList.Count == 0)
            {
                Console.WriteLine("Missing Requested: None");
            }
            else
            {
                Console.WriteLine("Missing Requested:");
                Console.WriteLine(string.Join(Environment.NewLine, missingNamesList));
            }
        }

        protected virtual void WriteMissingNames(Layer layer, IList<string> list)
        {
            if (layer is MissingLayer && !list.Contains(layer.SemVerName.Name))
            {
                list.Add(layer.SemVerName.Name);
            }
            foreach (var dependency in layer.Dependencies)
            {
                WriteMissingNames(dependency, list);
            }
        }
    }
}