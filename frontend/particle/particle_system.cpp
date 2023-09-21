
int32 ParticleSystem_create_particle(b2ParticleSystem* self, const b2ParticleDef& def) {
    return self->CreateParticle(def);
}
