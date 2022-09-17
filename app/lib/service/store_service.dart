import 'dart:convert';
import 'dart:io';

import 'package:path_provider/path_provider.dart';
import 'package:rusty_controller/main.dart';

class StoreService {
  /// Gets the store object in [defaultValue.name] if present and successful.
  ///
  /// Otherwise calls [save] with [defaultValue] and returns it.
  Future<dynamic> get({required StorableObject defaultValue}) async {
    try {
      final value =
          await _getRaw(defaultValue.storeName, defaultValue.fromJson);

      if (value == null) {
        log.i(
            "No store for file '${defaultValue.storeName}', saving default provided.");
        return await save(defaultValue);
      }

      log.v("Got file of store '${defaultValue.storeName}'");
      return Future.value(value);
    } catch (e) {
      log.w('Failed to get value for ${defaultValue.storeName}.', e);
      log.d('Writing default value provided.');
      return await save(defaultValue);
    }
  }

  // Saves, and returns [value] for convenience.
  Future<StorableObject> save(StorableObject value) async {
    final file = await _getStoreFile(value.storeName);
    final jsonContent = jsonEncode(value.toJson());

    await file.writeAsString(jsonContent);
    log.v('Stored object to file ${file.path}');

    return value;
  }

  /// Gets the store object in [storeName] mapping to [StorableObject]
  /// via [fromJson], or null if the file doesn't exist.
  Future<StorableObject?> _getRaw(String storeName,
      StorableObject Function(Map<String, dynamic>) fromJson) async {
    final file = await _getStoreFile(storeName);

    final jsonContent = jsonDecode(await file.readAsString());

    return fromJson(jsonContent);
  }

  Future<File> _getStoreFile(String storeName) async {
    final dir = await getApplicationDocumentsDirectory();
    final path =
        "${dir.path}\\rusty_controller\\stores\\${storeName}_effect.json";
    final file = File(path);

    await file.create(recursive: true);

    return file;
  }
}

abstract class StorableObject {
  String get storeName;

  StorableObject fromJson(Map<String, dynamic> json);

  Map<String, dynamic> toJson();
}
