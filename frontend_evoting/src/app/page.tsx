import HeroSection from '@/components/HeroSection';
import WorkflowSection from '@/components/WorkflowSection';
import AdminControlSection from '@/components/AdminControlSection';
import CoreInfrastructureSection from '@/components/CoreInfrastructureSection';
import NepalSolutionsSection from '@/components/NepalSolutionsSection';
import SecurityTrustSection from '@/components/SecurityTrustSection';
import CtaSection from '@/components/CtaSection';

export default function HomePage() {
  return (
    <>
      <HeroSection />
      <WorkflowSection />
      <AdminControlSection />
      <CoreInfrastructureSection />
      <NepalSolutionsSection />
      <SecurityTrustSection />
      <CtaSection />
    </>
  );
}
