install-gsettings:
	install -D target/gra-gen/org.example.SimpleExample.gschema.xml /usr/share/glib-2.0/schemas/org.example.SimpleExample.gschema.xml
	glib-compile-schemas /usr/share/glib-2.0/schemas

uninstall-gsettings:
	rm /usr/share/glib-2.0/schemas/org.example.SimpleExample.gschema.xml
	glib-compile-schemas /usr/share/glib-2.0/schemas
